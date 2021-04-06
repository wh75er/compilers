use super::Dfa;

use std::borrow::Cow;

type Nd = usize;
type Ed = (usize, usize, String);

impl<'a> dot::Labeller<'a, Nd, Ed> for Dfa {
    fn graph_id(&self) -> dot::Id<'a> {
        dot::Id::new("dfa").unwrap()
    }

    fn node_id(&self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n)).unwrap()
    }

    fn node_label(&self, n: &Nd) -> dot::LabelText {
        dot::LabelText::LabelStr(format!("{}", n).into())
    }

    fn edge_label(&self, e: &Ed) -> dot::LabelText {
        dot::LabelText::LabelStr(e.2.clone().into())
    }

    fn node_style(&self, n: &Nd) -> dot::Style {
        match self.is_terminal.contains(n) {
            true => dot::Style::Bold,
            _ => dot::Style::Solid
        }
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed> for Dfa {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        (0..self.states.len()).collect()
    }

    fn edges(&self) -> dot::Edges<'a, Ed> {
        let mut edges: Vec<Ed> = vec!();

        self.trans.iter().enumerate().for_each(|(state, to_states)| {
            to_states.iter().enumerate().for_each(|(c, to_state)| {
                edges.push((state, *to_state, self.alphabet[c].clone()));
            })
        });

        Cow::Owned(edges)
    }

    fn source(&self, e: &Ed) -> Nd {e.0}

    fn target(&self, e: &Ed) -> Nd {e.1}
}
