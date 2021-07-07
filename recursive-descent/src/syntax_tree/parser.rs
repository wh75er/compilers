use super::{TermType, Operations, SyntaxTree};

use std::error;
use std::iter::Peekable;

struct Parser<'a> {
    expr: &'a mut Peekable<std::slice::Iter<'a, &'a str>>,
}

impl Parser<'_> {
    fn math_expr(&mut self) -> Result<Box<SyntaxTree>, String> {
        Err("".into())
    }

    fn primary_expr(&mut self) -> Result<Box<SyntaxTree>, String> {
        let mut node = Box::new(SyntaxTree::new_node());

        let s = self.expr.peek().ok_or("Expected number, identifier or opening bracket. None of them were found")?;

        match TermType::from_string(s) {
            Some(TermType::NUMBER(v)) => {
                node.entry = TermType::NUMBER(v);
                self.expr.next();
                Ok(node)
            }
            Some(TermType::IDENTIFIER(v)) => {
                node.entry = TermType::IDENTIFIER(v);
                self.expr.next();
                Ok(node)
            }
            Some(TermType::OPERATION(Operations::LBRACKET)) => {
                self.expr.next();
                node = self.math_expr()?;
                if let Some(rbracket) = self.expr.peek() {
                    match Operations::from_string(rbracket) {
                        Some(Operations::RBRACKET) => Ok(node),
                        _ => Err("Expected closing bracket, found something else".into())
                    }
                } else {
                    Err("Expected closing bracket, found nothing".into())
                }
            }
            _ => Err("Expected number, identifier or opening bracket, found something else".into())
        }
    }
}

pub fn parse<'a>(tokenized_expr: Vec<&'a str>) -> Result<Box<SyntaxTree>, Box<dyn error::Error>> {
    #[cfg(debug_assertions)]
    println!("Tokenized expression: {:?}", tokenized_expr.iter().map(ToString::to_string));

    let mut parser = Parser {
        expr: &mut tokenized_expr.iter().peekable(),
    };

    let syntax_tree = parser.primary_expr();

    // #[cfg(debug_assertions)]
    // println!("Parsed expression: {:#?}", syntax_tree);

    Ok(syntax_tree.unwrap())
    // Err("BLUNK HERE REMOVE IT".into())
}
