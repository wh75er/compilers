pub mod dfa;
pub mod dfa_minimization;
mod draw;
pub mod utils;

use crate::syntax_tree::{GrammarType, Operations, SyntaxTree};

use std::collections::{HashMap, HashSet};
use std::fs::File;
use utils::{calculate_first_last_pos, generate_follow_pos, map_leaf};

#[derive(Debug)]
pub struct NodeWrapper<'a> {
    pub node: &'a Box<SyntaxTree>,
    pub left: Option<Box<NodeWrapper<'a>>>,
    pub right: Option<Box<NodeWrapper<'a>>>,
    pub leaf_index: Option<usize>,
    pub nullable: bool,
    pub first_pos: Vec<usize>,
    pub last_pos: Vec<usize>,
}

#[derive(Debug)]
pub struct Dfa {
    pub alphabet: Vec<String>,
    pub states: Vec<Vec<usize>>,
    pub trans: Vec<Vec<Option<usize>>>,
    pub is_terminal: HashSet<usize>,
}

impl Dfa {
    fn add_state(&mut self, new_state: &Vec<usize>, is_terminal: bool) {
        self.states.push(new_state.clone());
        self.trans.push(vec![None; self.alphabet.len()]);

        if is_terminal {
            self.is_terminal.insert(self.states.len() - 1);
        }
    }

    pub fn render_to(&self, output: &str) {
        let mut f = File::create(output).unwrap();
        dot::render(self, &mut f).unwrap()
    }
}

impl NodeWrapper<'_> {
    fn new(root: &Box<SyntaxTree>) -> Box<NodeWrapper> {
        let left_node = match &root.left {
            Some(n) => Some(NodeWrapper::new(&n)),
            _ => None,
        };
        let right_node = match &root.right {
            Some(n) => Some(NodeWrapper::new(&n)),
            _ => None,
        };

        let is_nullable = match root.entry {
            GrammarType::CHAR(_) => false,
            GrammarType::OPERATION(Operations::REPETITION) => true,
            GrammarType::OPERATION(Operations::OR) => {
                left_node.as_ref().map_or(false, |v| v.nullable)
                    || right_node.as_ref().map_or(false, |v| v.nullable)
            }
            GrammarType::OPERATION(Operations::CONCAT) => {
                left_node.as_ref().map_or(false, |v| v.nullable)
                    && right_node.as_ref().map_or(false, |v| v.nullable)
            }
            _ => false,
        };

        Box::new(NodeWrapper {
            node: root,
            left: left_node,
            right: right_node,
            leaf_index: None,
            nullable: is_nullable,
            first_pos: vec![],
            last_pos: vec![],
        })
    }

    fn calc_first_last_pos(&mut self) {
        calculate_first_last_pos(self);
    }

    fn gen_follow_pos(&self, size: usize) -> Vec<Vec<usize>> {
        let mut follow_pos = vec![Vec::<usize>::new(); size];

        generate_follow_pos(self, &mut follow_pos);

        follow_pos
    }

    fn numerate_leaves(&mut self) -> (Vec<String>, HashMap<usize, String>) {
        let mut leaf_counter = 0;
        let mut alphabet = vec![];
        let mut leaf_chars: HashMap<usize, String> = HashMap::new();

        map_leaf(self, &mut |v: &mut NodeWrapper| {
            v.leaf_index = Some(leaf_counter);

            match &v.node.entry {
                GrammarType::CHAR(s) => {
                    if s != Operations::TERMINATOR.as_string() {
                        alphabet.push(s.into());
                        leaf_chars.insert(leaf_counter, s.into());
                    }
                }
                _ => (),
            }

            leaf_counter += 1;
        });

        alphabet.sort();
        alphabet.dedup();

        (alphabet, leaf_chars)
    }
}
