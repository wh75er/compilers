pub mod parser;
mod utils;

#[derive(Debug)]
pub struct SyntaxTree {
    pub entry: GrammarType,
    pub left: Option<Box<SyntaxTree>>,
    pub right: Option<Box<SyntaxTree>>,
}

#[derive(Debug, PartialEq)]
pub enum GrammarType {
    OPERATION(Operations),
    CHAR(String),
    NULL,
}

#[derive(Debug, PartialEq)]
pub enum Operations {
    OR,
    REPETITION,
    CONCAT,
    LBRACKET,
    RBRACKET,
    TERMINATOR,
    ESCAPE,
}

impl SyntaxTree {
    fn new_node() -> SyntaxTree {
        SyntaxTree {
            entry: GrammarType::NULL,
            left: None,
            right: None,
        }
    }
}

impl Operations {
    fn from_char(c: &char) -> Option<Operations> {
        match c {
            '|' => Some(Operations::OR),
            '*' => Some(Operations::REPETITION),
            '.' => Some(Operations::CONCAT),
            '(' => Some(Operations::LBRACKET),
            ')' => Some(Operations::RBRACKET),
            '#' => Some(Operations::TERMINATOR),
            '\\' => Some(Operations::ESCAPE),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn from_string(s: &str) -> Option<Operations> {
        match s {
            "|" => Some(Operations::OR),
            "*" => Some(Operations::REPETITION),
            "." => Some(Operations::CONCAT),
            "(" => Some(Operations::LBRACKET),
            ")" => Some(Operations::RBRACKET),
            "#" => Some(Operations::TERMINATOR),
            "\\" => Some(Operations::ESCAPE),
            _ => None,
        }
    }

    pub fn as_string(&self) -> &'static str {
        match self {
            Operations::OR => "|",
            Operations::REPETITION => "*",
            Operations::CONCAT => ".",
            Operations::LBRACKET => "(",
            Operations::RBRACKET => ")",
            Operations::TERMINATOR => "#",
            Operations::ESCAPE => "\\",
        }
    }
}
