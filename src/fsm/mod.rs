pub mod dfa;
pub mod utils;

use crate::syntax_tree::{GrammarType, Operations, SyntaxTree};

use std::collections::HashMap;
use utils::{calculate_first_last_pos, generate_follow_pos, map_leaf};

#[derive(Debug)]
pub struct NodeWrapper<'a> {
    pub node: &'a Box<SyntaxTree>,
    pub left: Option<Box<NodeWrapper<'a>>>,
    pub right: Option<Box<NodeWrapper<'a>>>,
    pub leaf_index: Option<i32>,
    pub nullable: bool,
    pub first_pos: Vec<i32>,
    pub last_pos: Vec<i32>,
}

#[derive(Debug)]
pub struct Dfa {
    pub alphabet: Vec<String>,
    pub states: Vec<Vec<i32>>,
    pub trans: Vec<Vec<i32>>,
}

impl Dfa {
    fn add_state(&mut self, new_state: &Vec<i32>) {
        self.states.push(new_state.clone());
        self.trans.push(vec![0; self.alphabet.len()])
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

    fn gen_follow_pos(&self, size: usize) -> Vec<Vec<i32>> {
        let mut follow_pos = vec![Vec::<i32>::new(); size];

        generate_follow_pos(self, &mut follow_pos);

        follow_pos
    }

    fn numerate_leaves(&mut self) -> (Vec<String>, HashMap<i32, String>) {
        let mut leaf_counter = 0;
        let mut alphabet = vec![];
        let mut leaf_chars: HashMap<i32, String> = HashMap::new();

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
