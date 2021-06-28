pub mod transformations;

use std::collections::HashSet;

pub const EPSILON_SYMBOL: char = '&';

lazy_static! {
    static ref U_CODEPOINTS: HashSet<char> = {
        let mut h = HashSet::new();
        h.insert('\u{030c}');
        h.insert('\u{0320}');
        h.insert('\u{0337}');
        h
    };
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SymbolsKind {
    TERM,
    NONTERM,
    EPSILON,
}

/// Symbol is represented here
#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolsKind,
    pub value: String,
}

/// Production is represented here
#[derive(Debug, Clone)]
pub struct Production {
    replaced_symbol: Symbol,
    expression: Vec<Symbol>,
}

/// Grammar is represented here
#[derive(Debug)]
pub struct Grammar {
    non_terms: HashSet<String>,
    terms: HashSet<String>,
    productions: Vec<Production>,
    start: String,
}

impl Grammar {
    /// Returns a grammar with given parameters
    ///
    /// # Arguments
    ///
    /// * `non_terms` - Non-terminal symbols represented by &String
    ///
    /// * `terms` - Terminal symbols represented by &String
    pub fn new(
        non_terms: HashSet<String>,
        terms: HashSet<String>,
        prods: Vec<Production>,
        start: String,
    ) -> Grammar {
        Grammar {
            non_terms,
            terms,
            productions: prods.clone(),
            start,
        }
    }

    pub fn new_from_chars(
        non_terms: HashSet<char>,
        terms: HashSet<char>,
        prods: Vec<Production>,
        start: char,
    ) -> Grammar {
        Grammar {
            non_terms: non_terms.into_iter().map(|c| c.to_string()).collect(),
            terms: terms.into_iter().map(|c| c.to_string()).collect(),
            productions: prods,
            start: start.to_string(),
        }
    }
}

impl Production {
    /// Returns a production with given parameters
    ///
    /// # Arguments
    ///
    /// * `symbols` - Symbols which represent production rule. First element of Vec represents
    ///     left part of rule(replaced symbol), others represent a right part of the rule
    pub fn new(symbols: Vec<(SymbolsKind, String)>) -> Production {
        let symbols = symbols
            .into_iter()
            .map(|v| Symbol {
                kind: v.0,
                value: v.1,
            })
            .collect::<Vec<Symbol>>();
        let (first, elements) = symbols.split_first().expect("failed to split vector");
        Production {
            replaced_symbol: (*first).clone(),
            expression: elements.to_vec(),
        }
    }

    pub fn new_from_chars(symbols: Vec<(SymbolsKind, char)>) -> Production {
        let symbols = symbols
            .into_iter()
            .map(|v| Symbol {
                kind: v.0,
                value: v.1.to_string(),
            })
            .collect::<Vec<Symbol>>();
        let (first, elements) = symbols.split_first().expect("failed to split vector");
        Production {
            replaced_symbol: (*first).clone(),
            expression: elements.to_vec(),
        }
    }
}
