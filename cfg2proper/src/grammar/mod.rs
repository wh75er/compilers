pub mod transformations;
pub mod parser;

use std::collections::{ HashSet, HashMap };
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

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum SymbolsKind {
    TERM,
    NONTERM,
    EPSILON,
}

/// Symbol is represented here
#[derive(Debug, Clone, Hash)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

struct ProductionVec<'a>(&'a Vec<Production>);
struct SymbolVec<'a>(&'a Vec<Symbol>);

impl<'a> Display for SymbolVec<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut symbols_it = self.0.iter().peekable();
        while let Some(sym) = symbols_it.next() {
            if symbols_it.peek().is_some() {
                write!(f, "{} ", sym.value)?;
            } else {
                write!(f, "{}", sym.value)?;
            }
        }

        Ok(())
    }
}

impl<'a> Display for ProductionVec<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lhs_prods_map: HashMap<String, Vec<Vec<Symbol>>> = HashMap::new();

        for prod in self.0 {
            if let Some(v) = lhs_prods_map.get_mut(&prod.replaced_symbol.value) {
                v.push(prod.expression.clone());
            } else {
                lhs_prods_map.insert(prod.replaced_symbol.value.to_string(), vec![prod.expression.clone()]);
            }
        }

        for (lhs, rules) in lhs_prods_map {
            write!(f, "{} -> ", lhs)?;
            let mut rules_it = rules.iter().peekable();
            while let Some(rule) = rules_it.next() {
                if rules_it.peek().is_some() {
                    write!(f, "{} | ", SymbolVec(rule))?;
                } else {
                    write!(f, "{}", SymbolVec(rule))?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
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

        write!(f, "Productions: \n")?;
        write!(f, "{}\n", ProductionVec(&self.productions))?;

        write!(f, "Start: {}", self.start)?;

        Ok(())
    }
}
