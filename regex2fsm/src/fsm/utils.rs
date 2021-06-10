use super::{GrammarType, NodeWrapper, Operations};

pub fn map_leaf<F>(root: &mut NodeWrapper, callback: &mut F)
where
    F: FnMut(&mut NodeWrapper),
{
    root.left.as_mut().map(|n| map_leaf(n, callback));
    root.right.as_mut().map(|n| map_leaf(n, callback));

    if root.left.is_none() && root.right.is_none() {
        callback(root);
    }
}

pub fn calculate_first_last_pos(root: &mut NodeWrapper) -> (Vec<usize>, Vec<usize>) {
    if root.left.is_some() || root.right.is_some() {
        return match &root.node.entry {
            GrammarType::OPERATION(Operations::OR) => {
                let (l_first_pos, l_last_pos) = root
                    .left
                    .as_mut()
                    .map_or((vec![], vec![]), |v| calculate_first_last_pos(v));
                let (r_first_pos, r_last_pos) = root
                    .right
                    .as_mut()
                    .map_or((vec![], vec![]), |v| calculate_first_last_pos(v));

                root.first_pos.extend_from_slice(&l_first_pos);
                root.first_pos.extend_from_slice(&r_first_pos);

                root.last_pos.extend_from_slice(&l_last_pos);
                root.last_pos.extend_from_slice(&r_last_pos);

                (root.first_pos.clone(), root.last_pos.clone())
            }
            GrammarType::OPERATION(Operations::CONCAT) => {
                let (l_first_pos, _) = root
                    .left
                    .as_mut()
                    .map_or((vec![], vec![]), |v| calculate_first_last_pos(v));
                let (r_first_pos, r_last_pos) = root
                    .right
                    .as_mut()
                    .map_or((vec![], vec![]), |v| calculate_first_last_pos(v));

                root.first_pos.extend_from_slice(&l_first_pos);
                if root.left.as_ref().map_or(false, |v| v.nullable) {
                    root.first_pos.extend_from_slice(&r_first_pos);
                }

                root.last_pos.extend_from_slice(&r_last_pos);
                if root.right.as_ref().map_or(false, |v| v.nullable) {
                    root.last_pos.extend_from_slice(&l_first_pos);
                }

                (root.first_pos.clone(), root.last_pos.clone())
            }
            GrammarType::OPERATION(Operations::REPETITION) => {
                let (l_first_pos, l_last_pos) = root
                    .left
                    .as_mut()
                    .map_or((vec![], vec![]), |v| calculate_first_last_pos(v));

                root.first_pos.extend_from_slice(&l_first_pos);
                root.last_pos.extend_from_slice(&l_last_pos);
                (root.first_pos.clone(), root.last_pos.clone())
            }
            _ => panic!("There must be operation!"),
        };
    }

    match &root.node.entry {
        GrammarType::CHAR(_) => {
            let index = root.leaf_index.map_or(0, |v| v);

            root.first_pos.push(index);
            root.last_pos.push(index);
            (root.first_pos.clone(), root.last_pos.clone())
        }
        _ => panic!("Leaf must only contain char!"),
    }
}

pub fn generate_follow_pos(root: &NodeWrapper, follow_pos: &mut Vec<Vec<usize>>) {
    match root.node.entry {
        GrammarType::OPERATION(Operations::CONCAT) => {
            let l_last_pos = root.left.as_ref().map_or(vec![], |v| v.last_pos.clone());
            let r_first_pos = root.right.as_ref().map_or(vec![], |v| v.first_pos.clone());

            for v in l_last_pos.iter() {
                follow_pos[*v].extend_from_slice(&r_first_pos);
            }

            root.left
                .as_ref()
                .map(|v| generate_follow_pos(v, follow_pos));
            root.right
                .as_ref()
                .map(|v| generate_follow_pos(v, follow_pos));
        }
        GrammarType::OPERATION(Operations::REPETITION) => {
            for v in root.last_pos.iter() {
                follow_pos[*v].extend_from_slice(&root.first_pos);
            }

            root.left
                .as_ref()
                .map(|v| generate_follow_pos(v, follow_pos));
            root.right
                .as_ref()
                .map(|v| generate_follow_pos(v, follow_pos));
        }
        GrammarType::OPERATION(_) => {
            root.left
                .as_ref()
                .map(|v| generate_follow_pos(v, follow_pos));
            root.right
                .as_ref()
                .map(|v| generate_follow_pos(v, follow_pos));
        }
        _ => (),
    }
}
