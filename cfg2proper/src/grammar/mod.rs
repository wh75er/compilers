pub mod transformations;

use std::collections::HashSet;

pub const EPSILON_SYMBOL: &str = "&";
const NEW_START: &str = "$";

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
    expression: Vec<Symbol>
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
    /// * `non_terms` - Non-terminal symbols represented by String
    ///
    /// * `terms` - Terminal symbols represented by String
    pub fn new<T: ToString>(
        non_terms: &HashSet<T>,
        terms: &HashSet<T>,
        prods: &Vec<Production>,
        start: T,
    ) -> Grammar {
        Grammar {
            non_terms: non_terms.iter().map(|s| s.to_string()).collect(),
            terms: terms.iter().map(|s| s.to_string()).collect(),
            productions: prods.to_vec(),
            start: start.to_string()
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
    pub fn new<T: ToString>(
        symbols: &Vec<(SymbolsKind, T)>
    ) -> Production {
        let symbols = symbols.iter().map(|v| Symbol {
            kind: v.0,
            value: v.1.to_string()
        }).collect::<Vec<Symbol>>();
        let (first, elements) = symbols.split_first().expect("failed to split vector");
        Production {
            replaced_symbol: first.clone(),
            expression: elements.to_vec(),
        }
    }
}
