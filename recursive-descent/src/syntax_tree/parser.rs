use super::{TermType, Operations, SyntaxTree};

use std::error;
use std::iter::Peekable;
use crate::syntax_tree::is_valid_id;
use crate::syntax_tree::TermType::OPERATION;

struct Parser<'a> {
    expr: Box<Peekable<std::slice::Iter<'a, &'a str>>>,
}

pub fn parse<'a>(tokenized_expr: Vec<&'a str>) -> Result<Box<SyntaxTree>, Box<dyn error::Error>> {
    #[cfg(debug_assertions)]
    println!("Tokenized expression: {:?}", tokenized_expr.iter().map(ToString::to_string));

    let mut parser = Parser {
        expr: Box::new(tokenized_expr.iter().peekable()),
    };

    let syntax_tree = parser.block()?;

    if parser.expr.peek().is_some() {
        return Err("Syntax error occurred".into())
    }

    #[cfg(debug_assertions)]
    println!("Parsed expression: {:#?}", syntax_tree);

    Ok(syntax_tree)
}

impl Parser<'_> {
    fn block(&mut self) -> Result<Box<SyntaxTree>, String> {
        let s = self.expr.peek().ok_or("Expected begin, found nothing")?;

        if s != &&"begin" {
            return Err("Syntax error. Expected begin, found nothing".into());
        }

        self.expr.next();

        let node = self.operator_list()?;

        let s = self.expr.peek().ok_or("Expected end, found nothing")?;

        if s != &&"end" {
            return Err("Syntax error. Expected begin, found nothing".into());
        }

        self.expr.next();

        Ok(node)

    }

    fn operator_list(&mut self) -> Result<Box<SyntaxTree>, String> {
        // Check operator-list ::= operator
        let inner_node = self.operator()?;

        // Check factor ::= primary_expr factor'
        match self.operator_list_quote() {
            Ok(mut operator) => {
                operator.left = Some(inner_node);
                Ok(operator)
            },
            _ => Ok(inner_node)
        }
    }

    fn operator_list_quote(&mut self) -> Result<Box<SyntaxTree>, String> {
        let s = self.expr.peek().ok_or("Expected symbol ;, found nothing")?;

        match **s {
            ";" => {
                let iter_state = self.expr.clone();
                self.expr.next();
                // Check operator-list' ::= ';' operator
                return match self.operator() {
                    Ok(operator_node) => {
                        let mut node = Box::new(SyntaxTree::new_node());
                        node.entry = TermType::DELIMITER;
                        // check operator-list' ::= ';' operator operator-list'
                        if let Ok(mut right) = self.operator_list_quote() {
                            right.left = Some(operator_node);
                            node.right = Some(right);
                        } else {
                            node.right = Some(operator_node);
                        }
                        Ok(node)
                    },
                    Err(e) => {
                        self.expr = iter_state;
                        Err(e)
                    }
                }
            }
            _ => Err("Expected ;, found something else".into())
        }
    }

    fn operator(&mut self) -> Result<Box<SyntaxTree>, String> {
        let it_state = self.expr.clone();
        return match self.id() {
            Ok(id) => {
                let s = self.expr.peek().ok_or("Expected = sign. But nothing were found")?;
                match **s {
                    "=" => {
                        self.expr.next();
                        match self.expr() {
                            Ok(expr) => {
                                let mut node = Box::new(SyntaxTree::new_node());
                                node.entry = TermType::OPERATION(Operations::EQUAL);
                                node.left = Some(id);
                                node.right = Some(expr);
                                Ok(node)
                            },
                            Err(e) => Err(e)
                        }
                    },
                    _ => Err("Expected = sign, found something else".into())
                }
            },
            Err(e) => {
                self.expr = it_state;
                Err(e)
            }
        }
    }

    fn id(&mut self) -> Result<Box<SyntaxTree>, String> {
        let s = self.expr.peek().ok_or("Expected identifier([a-z]*) symbols. But nothing were found")?;

        if is_valid_id(&s.to_string()) {
            let mut node = Box::new(SyntaxTree::new_node());
            node.entry = TermType::ID(s.to_string());
            self.expr.next();
            return Ok(node)
        }

        return Err("Expected lowercase alphabetic signs. Found something else".into());
    }

    fn expr(&mut self) -> Result<Box<SyntaxTree>, String> {
        #[cfg(debug_assertions)]
        println!("Entering first expression");
        let l_expr_node = self.math_expr()?;
        #[cfg(debug_assertions)]
        println!("Exiting");

        let mut equality_sign_node = self.equality_sign()?;
        #[cfg(debug_assertions)]
        println!("Exiting equality_sign_node");

        #[cfg(debug_assertions)]
        println!("Entering left expression");
        let r_expr_node = self.math_expr()?;

        equality_sign_node.left = Some(l_expr_node);
        equality_sign_node.right = Some(r_expr_node);

        Ok(equality_sign_node)
    }

    fn math_expr(&mut self) -> Result<Box<SyntaxTree>, String> {
        #[cfg(debug_assertions)]
        println!("Entered to math-expr");
        // Check production math-expr ::= term | term math-expr'
        match self.term() {
            Ok(term_node) => {
                return match self.math_expr_quote() {
                    Ok(mut math_expr_node) => {
                        math_expr_node.left = Some(term_node);
                        Ok(math_expr_node)
                    },
                    Err(_) => {
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
        #[cfg(debug_assertions)]
        println!("Entered to math-expr'");
        let it_state = self.expr.clone();
        // Check math-expr' ::= add-sign term
        let mut add_sign_node = self.add_sign()?;

        return match self.term() {
            Ok(term_node) => {
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
            },
            Err(e) => {
                self.expr = it_state;
                Err(e)
            }
        }

    }

    fn term(&mut self) -> Result<Box<SyntaxTree>, String> {
        #[cfg(debug_assertions)]
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
        #[cfg(debug_assertions)]
        println!("Entered to term'");
        let it_state = self.expr.clone();
        // Check term' ::= multiplier-sign factor
        let mut multiplier_sign_node = self.multiplier_sign()?;

        return match self.factor() {
            Ok(factor_node) => {
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
            },
            Err(e) => {
                self.expr = it_state;
                Err(e)
            }
        }
    }

    fn factor(&mut self) -> Result<Box<SyntaxTree>, String> {
        #[cfg(debug_assertions)]
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
        #[cfg(debug_assertions)]
        println!("Entered to factor'");
        let s = self.expr.peek().ok_or("Expected symbol ^, found nothing")?;

        match **s {
            "^" => {
                let iter_state = self.expr.clone();
                self.expr.next();
                // Check factor' ::= '^' primary-expr
                return match self.primary_expr() {
                    Ok(primary_expr_node) => {
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
                    },
                    Err(e) => {
                        self.expr = iter_state;
                        Err(e)
                    }
                }
            }
            _ => Err("Expected ^, found something else".into())
        }
    }

    fn primary_expr(&mut self) -> Result<Box<SyntaxTree>, String> {
        #[cfg(debug_assertions)]
        println!("Entered to primary-expr");
        let mut node = Box::new(SyntaxTree::new_node());

        let s = self.expr.peek().ok_or("Expected number, identifier or opening bracket. None of them were found")?;

        #[cfg(debug_assertions)]
        println!("Primary expr symbol: {:?}", s);
        #[cfg(debug_assertions)]
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
                #[cfg(debug_assertions)]
                println!("Starting working on bracket");
                self.expr.next();
                node = self.math_expr()?;
                #[cfg(debug_assertions)]
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
                #[cfg(debug_assertions)]
                println!("Haven't found symbol");
                Err("Expected number, identifier or opening bracket, found something else".into())
            }
        }
    }

    fn add_sign(&mut self) -> Result<Box<SyntaxTree>, String> {
        #[cfg(debug_assertions)]
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
        #[cfg(debug_assertions)]
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
        #[cfg(debug_assertions)]
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
