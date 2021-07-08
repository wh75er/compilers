use super::{TermType, Operations, SyntaxTree};

use std::error;
use std::iter::Peekable;

struct Parser<'a> {
    expr: &'a mut Peekable<std::slice::Iter<'a, &'a str>>,
}

pub fn parse<'a>(tokenized_expr: Vec<&'a str>) -> Result<Box<SyntaxTree>, Box<dyn error::Error>> {
    #[cfg(debug_assertions)]
    println!("Tokenized expression: {:?}", tokenized_expr.iter().map(ToString::to_string));

    let mut parser = Parser {
        expr: &mut tokenized_expr.iter().peekable(),
    };

    let syntax_tree = parser.expr()?;

    #[cfg(debug_assertions)]
    println!("Parsed expression: {:#?}", syntax_tree);

    Ok(syntax_tree)
}

impl Parser<'_> {
    fn expr(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entering first expression");
        let l_expr_node = self.math_expr()?;
        println!("Exiting");

        let mut equality_sign_node = self.equality_sign()?;
        println!("Exiting equality_sign_node");

        println!("Entering left expression");
        let r_expr_node = self.math_expr()?;

        equality_sign_node.left = Some(l_expr_node);
        equality_sign_node.right = Some(r_expr_node);

        Ok(equality_sign_node)
    }

    fn math_expr(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entered to math-expr");
        // Check production math-expr ::= term | term math-expr'
        match self.term() {
            Ok(term_node) => {
                return match self.math_expr_quote() {
                    Ok(mut math_expr_node) => {
                        math_expr_node.left = Some(term_node);
                        Ok(math_expr_node)
                    },
                    Err(e) => {
                        Ok(term_node)
                    },
                }
            },
            _ => ()
        }

        // Check production math-expr ::= add-sign term | add-sign term math-expr'
        match self.add_sign() {
            Ok(mut add_sign_node) => {
                return match self.term() {
                    Ok(term_node) => {
                        match self.math_expr_quote() {
                            Ok(mut math_expr_node) => {
                                math_expr_node.left = Some(term_node);
                                add_sign_node.left = Some(math_expr_node);
                            },
                            _ => {
                                add_sign_node.left = Some(term_node);
                            }
                        }
                        Ok(add_sign_node)
                    },
                    Err(e) => Err(e)
                }
            },
            _ => ()
        }

        Err("Expected math expression, found something else".into())
    }

    fn math_expr_quote(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entered to math-expr'");
        // Check math-expr' ::= add-sign term
        let mut add_sign_node = self.add_sign()?;

        let term_node = self.term()?;

        // Check math-expr' ::= add-sign term math-expr'
        match self.math_expr_quote() {
            Ok(mut math_expr_node) => {
                math_expr_node.left = Some(term_node);
                add_sign_node.right = Some(math_expr_node);
                Ok(add_sign_node)
            },
            _ => {
                add_sign_node.right = Some(term_node);
                Ok(add_sign_node)
            }
        }
    }

    fn term(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entered to term");
        // Check term ::= factor
        let factor_node = self.factor()?;

        // Check term ::= factor term'
        match self.term_quote() {
            Ok(mut term_quote_node) => {
                term_quote_node.left = Some(factor_node);
                Ok(term_quote_node)
            },
            _ => Ok(factor_node)
        }
    }

    fn term_quote(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entered to term'");
        // Check term' ::= multiplier-sign factor
        let mut multiplier_sign_node = self.multiplier_sign()?;

        let factor_node = self.factor()?;

        // Check term' ::= multiplier-sign factor term'
        match self.term_quote() {
            Ok(mut inner_term_node) => {
                inner_term_node.left = Some(factor_node);
                multiplier_sign_node.right = Some(inner_term_node);
                Ok(multiplier_sign_node)
            },
            _ => {
                multiplier_sign_node.right = Some(factor_node);
                Ok(multiplier_sign_node)
            }
        }
    }

    fn factor(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entered to factor");
        // Check factor ::= primary_expr
        let inner_node = self.primary_expr()?;

        // Check factor ::= primary_expr factor'
        match self.factor_quote() {
            Ok(mut power) => {
                power.left = Some(inner_node);
                Ok(power)
            },
            _ => Ok(inner_node)
        }
    }

    fn factor_quote(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entered to factor'");
        let s = self.expr.peek().ok_or("Expected symbol ^, found nothing")?;

        match **s {
            "^" => {
                self.expr.next();
                // Check factor' ::= '^' primary-expr
                let primary_expr_node = self.primary_expr()?;
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::POWER);
                // check factor' ::= '^' primary-expr factor'
                if let Ok(mut right) = self.factor_quote() {
                    right.left = Some(primary_expr_node);
                    node.right = Some(right);
                } else {
                    node.right = Some(primary_expr_node);
                }
                Ok(node)
            }
            _ => Err("Expected ^, found something else".into())
        }
    }

    fn primary_expr(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entered to primary-expr");
        let mut node = Box::new(SyntaxTree::new_node());

        let s = self.expr.peek().ok_or("Expected number, identifier or opening bracket. None of them were found")?;

        println!("Primary expr symbol: {:?}", s);
        println!("Primary expr TermType: {:?}", TermType::from_string(s));

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
                println!("Starting working on bracket");
                self.expr.next();
                node = self.math_expr()?;
                println!("Symbol after math_expr in brackets: {:?}", self.expr.peek());
                if let Some(rbracket) = self.expr.peek() {
                    match Operations::from_string(rbracket) {
                        Some(Operations::RBRACKET) => {
                            self.expr.next();
                            Ok(node)
                        },
                        _ => Err("Expected closing bracket, found something else".into())
                    }
                } else {
                    Err("Expected closing bracket, found nothing".into())
                }
            }
            _ => {
                println!("Haven't found symbol");
                Err("Expected number, identifier or opening bracket, found something else".into())
            }
        }
    }

    fn add_sign(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entered to add-sign");
        let s = self.expr.peek().ok_or("Expected + or - signs. None of them were found")?;

        match **s {
            "+" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::ADDITION);
                Ok(node)
            },
            "-" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::SUBTRACTION);
                Ok(node)
            }
            _ => Err("Expected + or - signs. Found something else".into())
        }
    }

    fn multiplier_sign(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entered to multiplier-sign");
        let s = self.expr.peek().ok_or("Expected *, / or %  signs. None of them were found")?;

        match **s {
            "*" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::MULTIPLICATION);
                Ok(node)
            },
            "/" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::DIVISION);
                Ok(node)
            },
            "%" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::MODULO);
                Ok(node)
            },
            _ => Err("Expected *, / or % signs. Found something else".into())
        }
    }

    fn equality_sign(&mut self) -> Result<Box<SyntaxTree>, String> {
        println!("Entered to equality-sign");
        let s = self.expr.peek().ok_or("Expected <, <=, =, >=, >, <> signs. None of them were found")?;

        match **s {
            "<" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::LESS);
                Ok(node)
            },
            "<=" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::LE);
                Ok(node)
            },
            "=" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::EQUAL);
                Ok(node)
            },
            ">=" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::GE);
                Ok(node)
            },
            ">" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::GREATER);
                Ok(node)
            },
            "<>" => {
                self.expr.next();
                let mut node = Box::new(SyntaxTree::new_node());
                node.entry = TermType::OPERATION(Operations::NOTEQUAL);
                Ok(node)
            },
            _ => Err("Expected <, <=, =, >=, >, <> signs. Found something else".into())
        }
    }
}
