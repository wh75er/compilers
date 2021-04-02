use super::{
    SyntaxTree,
    Operations,
    GrammarType,
};

use std::error;
use std::iter::Peekable;
use std::str::Chars;
use std::mem;
use crate::syntax_tree::GrammarType::OPERATION;

struct Parser<'a> {
    expr: &'a mut Peekable<Chars<'a>>,
}

impl Parser<'_> {
    fn regex(&mut self) -> Box<SyntaxTree> {
        let inner_node = self.term();

        let c = self.expr.peek();

        match c {
            Some(c) => match Operations::from_char(c) {
                Some(Operations::OR) => {
                    self.expr.next();
                    let mut node = Box::new(SyntaxTree::new_node());
                    node.left = Some(inner_node);
                    node.entry = GrammarType::OPERATION(Operations::OR);
                    node.right = Some(self.regex());
                    node
                },
                _ => inner_node,
            }
            _ => inner_node
        }
    }

    fn term(&mut self) -> Box<SyntaxTree> {
        let inner_node = self.concat();

        let c = self.expr.peek();

        match c {
            Some(c) => match Operations::from_char(c) {
                Some(Operations::AND) => {
                    self.expr.next();
                    let mut node = Box::new(SyntaxTree::new_node());
                    node.left = Some(inner_node);
                    node.entry = GrammarType::OPERATION(Operations::AND);
                    node.right = Some(self.term());
                    node
                },
                _ => inner_node
            }
            _ => inner_node
        }
    }

    fn concat(&mut self) -> Box<SyntaxTree> {
        let inner_node = self.factor();

        let c = self.expr.peek();

        match c {
            Some(c) => match Operations::from_char(c) {
                Some(Operations::CONCAT) => {
                    self.expr.next();
                    let mut node = Box::new(SyntaxTree::new_node());
                    node.left = Some(inner_node);
                    node.entry = GrammarType::OPERATION(Operations::CONCAT);
                    node.right = Some(self.concat());
                    node
                },
                _ => inner_node
            }
            _ => inner_node
        }
    }

    fn factor(&mut self) -> Box<SyntaxTree> {
        let inner_node = self.base();

        let c = self.expr.peek();

        match c {
            Some(c) => match Operations::from_char(c) {
                Some(Operations::REPETITION) => {
                    self.expr.next();
                    let mut node = Box::new(SyntaxTree::new_node());
                    node.left = Some(inner_node);
                    node.entry = GrammarType::OPERATION(Operations::REPETITION);
                    node
                }
                _ => inner_node
            }
            _ => inner_node
        }
    }

    fn base(&mut self) -> Box<SyntaxTree> {
        let mut node = Box::new(SyntaxTree::new_node());

        let c = self.expr.peek();

        match c {
            Some(c) => match Operations::from_char(c) {
                Some(Operations::ESCAPE) => {
                    self.expr.next();
                    node.entry = GrammarType::CHAR(self.expr.peek().unwrap().to_string());
                    self.expr.next();
                    node
                },
                Some(Operations::LBRACKET) => {
                    self.expr.next();
                    node = self.regex();
                    self.expr.next();
                    node
                },
                _ => {
                    node.entry = GrammarType::CHAR(self.expr.peek().unwrap().to_string());
                    self.expr.next();
                    node
                }
            }
            _ => node
        }
    }
}

pub fn parse<'a>(regex: &'a String) -> Result<SyntaxTree, Box<dyn error::Error>> {
    validate_regex(&regex)?;

    let regex = extend_concat_op(regex);

    println!("Extended regex: {}", regex);

    let mut parser = Parser{expr: &mut regex.chars().peekable()};

    let syntax_tree = parser.regex();

    println!("Parsed expression: {:?}", syntax_tree);

    Ok(SyntaxTree{entry: GrammarType::NULL, left: None, right: None})
}

fn extend_concat_op(regex: &String) -> String {
    let mut result = String::from("");

    let regex = String::from(regex) + Operations::TERMINATOR.as_string();

    let regex_chars: Vec<char> = regex.chars().collect();

    for (i, c) in regex_chars.iter().enumerate() {
        let next_char = regex_chars.get(i+1);

        result += &c.to_string();

        match Operations::from_char(c) {
            Some(v) => match v {
                Operations::REPETITION => result += Operations::CONCAT.as_string(),
                _ => ()
            },
            None => match next_char {
                Some(v) => match Operations::from_char(v) {
                    Some(v) => match v {
                        Operations::TERMINATOR => result += Operations::CONCAT.as_string(),
                        Operations::LBRACKET => result += Operations::CONCAT.as_string(),
                        Operations::ESCAPE => result += Operations::CONCAT.as_string(),
                        _ => ()
                    },
                    None => result += Operations::CONCAT.as_string()
                },
                None => ()
            }
        }
    }

    result
}

fn validate_regex(regex: &String) -> Result<(), Box<dyn error::Error>> {
    let forbidden_symbols = vec!{
        Operations::CONCAT.as_string(),
        Operations::TERMINATOR.as_string()
    };

    for c in forbidden_symbols.iter() {
        if regex.find(*c).is_some() {
            return Err((String::from("Symbol ") + *c + " is not allowed").into());
        }
    }

    Ok(())
}
