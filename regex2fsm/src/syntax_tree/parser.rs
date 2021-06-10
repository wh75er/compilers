use super::{utils::extend_concat_op, utils::validate_regex, GrammarType, Operations, SyntaxTree};

use std::error;
use std::iter::Peekable;
use std::str::Chars;

struct Parser<'a> {
    expr: &'a mut Peekable<Chars<'a>>,
}

impl Parser<'_> {
    fn regex(&mut self) -> Box<SyntaxTree> {
        let inner_node = self.concat();

        let c = self.expr.peek();

        match c.and_then(Operations::from_char) {
            Some(Operations::OR) => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.left = Some(inner_node);
                node.entry = GrammarType::OPERATION(Operations::OR);
                node.right = Some(self.regex());
                node
            }
            _ => inner_node,
        }
    }

    fn concat(&mut self) -> Box<SyntaxTree> {
        let inner_node = self.factor();

        let c = self.expr.peek();

        match c.and_then(Operations::from_char) {
            Some(Operations::CONCAT) => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.left = Some(inner_node);
                node.entry = GrammarType::OPERATION(Operations::CONCAT);
                node.right = Some(self.concat());
                node
            }
            _ => inner_node,
        }
    }

    fn factor(&mut self) -> Box<SyntaxTree> {
        let inner_node = self.base();

        let c = self.expr.peek();

        match c.and_then(Operations::from_char) {
            Some(Operations::REPETITION) => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.left = Some(inner_node);
                node.entry = GrammarType::OPERATION(Operations::REPETITION);
                node
            }
            _ => inner_node,
        }
    }

    fn base(&mut self) -> Box<SyntaxTree> {
        let mut node = Box::new(SyntaxTree::new_node());

        let c = self.expr.peek();

        match c.and_then(Operations::from_char) {
            Some(Operations::ESCAPE) => {
                self.expr.next();
                node.entry = GrammarType::CHAR(self.expr.peek().unwrap().to_string());
                self.expr.next();
                node
            }
            Some(Operations::LBRACKET) => {
                self.expr.next();
                node = self.regex();
                self.expr.next();
                node
            }
            _ => {
                node.entry = GrammarType::CHAR(self.expr.peek().unwrap().to_string());
                self.expr.next();
                node
            }
        }
    }
}

pub fn parse<'a>(regex: &'a String) -> Result<Box<SyntaxTree>, Box<dyn error::Error>> {
    validate_regex(&regex)?;

    let regex = extend_concat_op(regex);

    #[cfg(debug_assertions)]
    println!("Extended regex: {}", regex);

    let mut parser = Parser {
        expr: &mut regex.chars().peekable(),
    };

    let syntax_tree = parser.regex();

    #[cfg(debug_assertions)]
    println!("Parsed expression: {:#?}", syntax_tree);

    Ok(syntax_tree)
}
