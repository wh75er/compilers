use super::{Dfa, NodeWrapper, SyntaxTree};

use std::collections::{HashMap, HashSet};

pub fn transform(root: Box<SyntaxTree>) -> Dfa {
    let mut wrapper = NodeWrapper::new(&root);

    let (alphabet, leaf_chars) = wrapper.numerate_leaves();

    wrapper.calc_first_last_pos();
    let follow_pos = wrapper.gen_follow_pos(leaf_chars.len() + 1);

    #[cfg(debug_assertions)]
    println!("wrapper : {:#?}", wrapper);
    #[cfg(debug_assertions)]
    println!("follow pos : {:#?}", follow_pos);

    let mut dfa = Dfa {
        alphabet,
        states: vec![],
        trans: vec![],
        is_terminal: HashSet::new(),
    };

    dfa.add_state(&wrapper.first_pos.clone(), wrapper.first_pos.contains(&leaf_chars.len()));

    let mut row = 0;

    while row < dfa.states.len() {
        for col in 0..dfa.alphabet.len() {
            let curr_char = &dfa.alphabet[col];
            let curr_state = &dfa.states[row];
            let (new_state, is_terminal) =
                form_state(curr_char, curr_state, &follow_pos, &leaf_chars);
            match match_state(&dfa.states, &new_state) {
                Some(v) => {
                    dfa.trans[row][col] = Some(v);
                }
                _ => {
                    if new_state.is_empty() {
                        continue;
                    }
                    dfa.add_state(&new_state, is_terminal);
                    dfa.trans[row][col] = Some(dfa.states.len() - 1);
                }
            }
        }

        row += 1;
    }

    dfa
}

fn form_state(
    curr_char: &str,
    curr_state: &Vec<usize>,
    follow_pos: &Vec<Vec<usize>>,
    leaf_chars: &HashMap<usize, String>,
) -> (Vec<usize>, bool) {
    let mut result = vec![];

    for state_value in curr_state.iter() {
        if leaf_chars
            .get(state_value)
            .map_or(false, |v| v == curr_char)
        {
            result.extend_from_slice(&follow_pos[*state_value as usize]);
            result.sort();
            result.dedup();
        }
    }

    let mut is_terminal = false;

    if result.contains(&leaf_chars.len()) {
        is_terminal = true;
    }

    (result, is_terminal)
}

fn match_state(states: &Vec<Vec<usize>>, new_state: &Vec<usize>) -> Option<usize> {
    let mut b_vec = new_state.clone();
    b_vec.sort();

    for (i, state) in states.iter().enumerate() {
        let mut a_vec = state.clone();
        a_vec.sort();

        if a_vec.len() == b_vec.len() && a_vec.iter().zip(&b_vec).all(|(a, b)| a == b) {
            return Some(i);
        }
    }

    None
}
