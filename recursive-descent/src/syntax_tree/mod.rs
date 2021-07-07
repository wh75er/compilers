pub mod parser;

#[derive(Debug)]
pub struct SyntaxTree {
    pub entry: TermType,
    pub left: Option<Box<SyntaxTree>>,
    pub right: Option<Box<SyntaxTree>>,
}

#[derive(Debug, PartialEq)]
pub enum Identifiers {
    SingleQuote
}

#[derive(Debug, PartialEq)]
pub enum TermType {
    OPERATION(Operations),
    NUMBER(String),
    IDENTIFIER(Identifiers),
    NULL,
}

#[derive(Debug, PartialEq)]
pub enum Operations {
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    MODULO,
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
}

impl Operations {
    fn from_string(s: &str) -> Option<Operations> {
        match s {
            "+" => Some(Operations::ADDITION),
            "-" => Some(Operations::SUBTRACTION),
            "*" => Some(Operations::MULTIPLICATION),
            "%" => Some(Operations::MODULO),
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

        None
    }
}
