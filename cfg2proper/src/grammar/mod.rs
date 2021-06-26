pub mod transformations;

use std::collections::HashSet;

pub const EPSILON_SYMBOL: char = '&';
const NEW_START: char = '$';

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
    pub value: char,
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
    non_terms: HashSet<char>,
    terms: HashSet<char>,
    productions: Vec<Production>,
    start: char,
}

impl Grammar {
    /// Returns a grammar with given parameters
    ///
    /// # Arguments
    ///
    /// * `non_terms` - Non-terminal symbols represented by char
    ///
    /// * `terms` - Terminal symbols represented by char
    pub fn new(
        non_terms: HashSet<char>,
        terms: HashSet<char>,
        prods: Vec<Production>,
        start: char,
    ) -> Grammar {
        Grammar {
            non_terms,
            terms,
            productions: prods,
            start
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
    pub fn new(
        symbols: Vec<(SymbolsKind, char)>
    ) -> Production {
        let symbols = symbols.into_iter().map(|v| Symbol {
            kind: v.0,
            value: v.1
        }).collect::<Vec<Symbol>>();
        let (first, elements) = symbols.split_first().expect("failed to split vector");
        Production {
            replaced_symbol: (*first).clone(),
            expression: elements.to_vec(),
        }
    }
}
