pub mod transformations;

use std::collections::HashSet;
use std::fmt::Display;

pub const EPSILON_SYMBOL: char = '&';

lazy_static! {
    static ref U_CODEPOINTS: HashSet<char> = {
        let mut h = HashSet::new();
        h.insert('\u{030c}');
        h.insert('\u{0320}');
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
    pub replaced_symbol: Symbol,
    pub expression: Vec<Symbol>,
}

/// Grammar is represented here
#[derive(Debug)]
pub struct Grammar {
    pub non_terms: HashSet<String>,
    pub terms: HashSet<String>,
    pub productions: Vec<Production>,
    pub start: String,
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

fn display_hashset(f: &mut std::fmt::Formatter<'_>, h: &HashSet<String>) -> std::fmt::Result {
    for s in h.iter() {
        write!(f, "{} ", s)?;
    }
    write!(f, "\n")?;

    Ok(())
}

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        write!(f, "Non-Terminals: ")?;
        display_hashset(f, &self.non_terms)?;
        write!(f, "Terminals: ")?;
        display_hashset(f, &self.terms)?;
        write!(f, "Productions:\n")?;

        for prod in self.productions.iter() {
            write!(f, "\t{} -> ", prod.replaced_symbol.value)?;
            // write!(f, "{:?}\n", prod.expression);
            for sym in prod.expression.iter().map(|sym| &sym.value) {
                write!(f, "{} ", sym)?;
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")?;
        write!(f, "Start: {}", self.start)?;

        Ok(())
    }
}
