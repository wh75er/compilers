pub mod parser;
mod draw;

use std::fs::File;

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    pub entry: TermType,
    pub left: Option<Box<SyntaxTree>>,
    pub right: Option<Box<SyntaxTree>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Identifiers {
    SingleQuote
}

#[derive(Debug, PartialEq, Clone)]
pub enum TermType {
    OPERATION(Operations),
    NUMBER(String),
    IDENTIFIER(Identifiers),
    ID(String),
    DELIMITER,
    NULL,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operations {
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    MODULO,
    DIVISION,
    LESS,
    LE,
    EQUAL,
    GE,
    GREATER,
    NOTEQUAL,
    POWER,
    LBRACKET,
    RBRACKET,
}

impl SyntaxTree {
    fn new_node() -> SyntaxTree {
        SyntaxTree {
            entry: TermType::NULL,
            left: None,
            right: None,
        }
    }
    pub fn render_to(&self, output: &str) {
        let mut f = File::create(output).unwrap();
        dot::render(self, &mut f).unwrap()
    }
}

impl Operations {
    fn from_string(s: &str) -> Option<Operations> {
        match s {
            "+" => Some(Operations::ADDITION),
            "-" => Some(Operations::SUBTRACTION),
            "*" => Some(Operations::MULTIPLICATION),
            "%" => Some(Operations::MODULO),
            "/" => Some(Operations::DIVISION),
            "<" => Some(Operations::LESS),
            "<=" => Some(Operations::LE),
            "=" => Some(Operations::EQUAL),
            ">=" => Some(Operations::GE),
            ">" => Some(Operations::GREATER),
            "<>" => Some(Operations::NOTEQUAL),
            "^" => Some(Operations::POWER),
            "(" => Some(Operations::LBRACKET),
            ")" => Some(Operations::RBRACKET),
            _ => None,
        }
    }

    pub fn as_string(&self) -> &'static str {
        match self {
            Operations::ADDITION => "+",
            Operations::SUBTRACTION => "-",
            Operations::MULTIPLICATION => "*",
            Operations::MODULO => "%",
            Operations::DIVISION => "/",
            Operations::LESS => "<",
            Operations::LE => "<=",
            Operations::EQUAL => "=",
            Operations::GE => ">=",
            Operations::GREATER => ">",
            Operations::NOTEQUAL => "<>",
            Operations::POWER => "^",
            Operations::LBRACKET => "(",
            Operations::RBRACKET => ")",
        }
    }
}

impl Identifiers {
    fn from_string(s: &str) -> Option<Identifiers> {
        match s {
            "\'" => Some(Identifiers::SingleQuote),
            _ => None
        }
    }

    fn as_string(&self) -> &'static str {
        match self {
            Identifiers::SingleQuote => "\'",
        }
    }
}

fn is_valid_id(s: &String) -> bool {
    let valid_symbols = s.chars().filter(|c| c.is_alphabetic() && c.is_lowercase()).collect::<Vec<_>>();

    valid_symbols.len() == s.len()
}

impl TermType {
    fn from_string(s: &str) -> Option<TermType> {
        if let Some(op) = Operations::from_string(s) {
            return Some(TermType::OPERATION(op));
        }

        if let Ok(_) = s.parse::<i64>() {
            return Some(TermType::NUMBER(s.to_string()));
        }

        if let Some(id) = Identifiers::from_string(s) {
            return Some(TermType::IDENTIFIER(id))
        }

        if is_valid_id(&s.to_string()) {
            return Some(TermType::ID(s.to_string()));
        }

        if s == ";" {
            return Some(TermType::DELIMITER)
        }

        None
    }

    fn as_string(&self) -> String {
        match self {
            TermType::OPERATION(op) => {
                op.as_string().to_string()
            },
            TermType::IDENTIFIER(id) => {
                id.as_string().to_string()
            },
            TermType::NUMBER(num) => {
                num.to_string()
            },
            TermType::NULL => {
                "null".to_string()
            }
            TermType::ID(id) => {
                id.to_string()
            },
            TermType::DELIMITER => {
                ";".to_string()
            }
        }
    }
}
