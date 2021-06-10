mod transformations;

use std::collections::HashSet;

const EPSILON_SYMBOL: &str = "&";

#[derive(Debug, Copy, Clone)]
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
    pub fn new(
        non_terms: &HashSet<String>,
        terms: &HashSet<String>,
        prods: &Vec<Production>,
        start: String,
    ) -> Grammar {
        Grammar {
            non_terms: non_terms.clone(),
            terms: terms.clone(),
            productions: prods.to_vec(),
            start,
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
        symbols: &Vec<(SymbolsKind, String)>
    ) -> Production {
        let symbols = symbols.iter().map(|v| match v.1.to_string() {
            EPSILON_SYMBOL => Symbol {
                kind: SymbolsKind::EPSILON,
                value: v.1.to_string()
            },
            _ => Symbol {
                kind: v.0,
                value: v.1.to_string()
            }
        }).collect::<Vec<Symbol>>().to_vec();
        let (first, elements) = symbols.split_first().expect("failed to split vector");
        Production {
            replaced_symbol: first.clone(),
            expression: elements.to_vec(),
        }
    }
}
