use crate::syntax_tree::{ SyntaxTree, TermType };
use std::borrow::Cow;

type Nd = (usize, TermType);
type Ed = (usize, usize);

impl<'a> dot::Labeller<'a, Nd, Ed> for SyntaxTree {
    fn graph_id(&self) -> dot::Id<'a> {
        dot::Id::new("syntaxtree").unwrap()
    }

    fn node_id(&self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n.0.to_string())).unwrap()
    }

    fn node_label(&self, n: &Nd) -> dot::LabelText {
        dot::LabelText::LabelStr(format!("{}", n.1.as_string()).into())
    }
}


impl<'a> dot::GraphWalk<'a, Nd, Ed> for SyntaxTree {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        let mut nodes: Vec<TermType> = vec![];
        let mut stack: Vec<Option<Box<SyntaxTree>>> = vec![];

        stack.push(Some(Box::new((*self).clone())));

        while !stack.is_empty() {
            match stack.pop().unwrap() {
                Some(node) => {
                    stack.push(node.right);
                    stack.push(node.left);
                    nodes.push(node.entry);
                },
                _ => ()
            }
        }

        nodes.iter().cloned().enumerate().collect()
    }

    fn edges(&self) -> dot::Edges<'a, Ed> {
        let mut edges: Vec<Ed> = vec![];
        let mut stack: Vec<(Option<Box<SyntaxTree>>, usize)> = vec![];

        let mut counter: usize = 0;

        stack.push((Some(Box::new((*self).clone())), counter.clone()));

        while !stack.is_empty() {
            let node_with_parent = stack.pop().unwrap();
            let node = node_with_parent.0;
            let parent_idx = node_with_parent.1;
            match node {
                Some(node) => {
                    stack.push((node.right, counter.clone()));
                    stack.push((node.left, counter.clone()));
                    if counter != 0 {
                        edges.push((parent_idx, counter.clone()));
                    }
                    counter += 1;
                },
                _ => ()
            }
        }

        Cow::Owned(edges)
    }

    fn source(&self, e: &Ed) -> Nd { self.nodes().iter().find(|x| x.0 == e.0).unwrap().clone() }

    fn target(&self, e: &Ed) -> Nd { self.nodes().iter().find(|x| x.0 == e.1).unwrap().clone() }
}
