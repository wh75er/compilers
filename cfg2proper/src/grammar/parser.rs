use crate::grammar::{Grammar, Production, Symbol, SymbolsKind, EPSILON_SYMBOL};
use std::error::Error;
use std::{ fs, fmt };
use std::collections::HashSet;
use std::io::prelude::*;

#[derive(Debug)]
enum ParseError {
    NonTermsNNotFound,
    NonTermsNotFound,
    TermsNNotFound,
    TermsNotFound,
    ProductionsNNotFound,
    InvalidProductionStart,
    InvalidProduction,
    ExpectedProductionNotFound,
    StartSymbolNotFound,
    FailedConvertIntoSymbol,
    InvalidStartSymbol,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::NonTermsNNotFound =>
                write!(f, "number of non-terms was not provided"),
            ParseError::NonTermsNotFound =>
                write!(f, "number of provided non-terms is not equal to expected number"),
            ParseError::TermsNNotFound =>
                write!(f, "number of terms was not provided"),
            ParseError::TermsNotFound =>
                write!(f, "number of provided terms is not equal to expected number"),
            ParseError::ProductionsNNotFound =>
                write!(f, "number of provided productions is not equal to expected number"),
            ParseError::InvalidProductionStart =>
                write!(f, "production starts with term"),
            ParseError::InvalidProduction =>
                write!(f, "production consists of unknown symbols(neither terms or non-terms)"),
            ParseError::ExpectedProductionNotFound =>
                write!(f, "number of provided productions is less than expected"),
            ParseError::StartSymbolNotFound =>
                write!(f, "start symbol was not found(if it exists try to check expected productions number and actual number)"),
            ParseError::FailedConvertIntoSymbol =>
                write!(f, "failed to convert string into symbol(neither terminal, neither non-terminal or epsilon)"),
            ParseError::InvalidStartSymbol =>
                write!(f, "start symbol must be non-terminal"),
        }
    }
}

impl Error for ParseError {}

pub fn parse_from_file(filename: &String) -> Result<Grammar, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let g = parse(&content)?;

    Ok(g)
}

fn parse(content: &String) -> Result<Grammar, Box<dyn Error>> {
    let mut rows = content.split('\n');

    let non_terms_set = parse_non_terms(&mut rows)?;

    let terms_set = parse_terms(&mut rows)?;

    let productions = parse_productions(&mut rows, &non_terms_set, &terms_set)?;

    let start = parse_start(&mut rows, &non_terms_set)?;

    Ok(Grammar::new(non_terms_set, terms_set, productions, start))
}

fn parse_non_terms<'a, I>(it: &mut I) -> Result<HashSet<String>, Box<dyn Error>>
    where I: Iterator<Item = &'a str>,
{
    let non_term_n = it.next().ok_or(ParseError::NonTermsNNotFound)?;
    let non_term_n = non_term_n.parse::<usize>()?;

    let non_terms_raw = it.next().ok_or(ParseError::NonTermsNotFound)?;

    let non_terms_set: HashSet<String> = non_terms_raw.split(' ')
        .map(ToString::to_string)
        .collect();

    if non_terms_set.len() != non_term_n {
        return Err(ParseError::NonTermsNotFound.into());
    }

    Ok(non_terms_set)
}

fn parse_terms<'a, I>(it: &mut I) -> Result<HashSet<String>, Box<dyn Error>>
    where I: Iterator<Item = &'a str>,
{
    let term_n = it.next().ok_or(ParseError::TermsNNotFound)?;
    let term_n = term_n.parse::<usize>()?;

    let terms_raw = it.next().ok_or(ParseError::TermsNotFound)?;

    let terms_set: HashSet<String> = terms_raw.split(' ')
        .map(ToString::to_string)
        .collect();

    if terms_set.len() != term_n {
        return Err(ParseError::TermsNotFound.into());
    }

    Ok(terms_set)
}

fn parse_productions<'a, I>(it: &mut I, non_terms: &HashSet<String>, terms: &HashSet<String>) -> Result<Vec<Production>, Box<dyn Error>>
    where I: Iterator<Item = &'a str>,
{
    let production_n = it.next().ok_or(ParseError::ProductionsNNotFound)?;
    let production_n = production_n.parse::<usize>()?;

    let mut productions: Vec<Production> = vec![];

    for _i in 0..production_n {
        let production = it.next().ok_or(ParseError::ExpectedProductionNotFound)?;
        let production_symbols: Vec<Symbol> = production.split(' ')
            .map(|s| convert_string_to_symbol(&s.to_string(), non_terms, terms))
            .collect::<Result<Vec<_>, _>>()?;

        if production_symbols.len() < 2 {
            return Err(ParseError::InvalidProduction.into());
        }

        if production_symbols[0].kind != SymbolsKind::NONTERM {
            return Err(ParseError::InvalidProductionStart.into());
        }

        productions.push(
            Production::new(production_symbols.into_iter().map(|sym| (sym.kind, sym.value)).collect::<Vec<_>>())
        );
    }

    Ok(productions)
}

fn parse_start<'a, I>(it: &mut I, non_terms: &HashSet<String>) -> Result<String, Box<dyn Error>>
    where I: Iterator<Item = &'a str>,
{
    let start = it.next().ok_or(ParseError::StartSymbolNotFound)?;

    if !non_terms.contains(start) {
        return Err(ParseError::InvalidStartSymbol.into());
    }

    Ok(start.to_string())
}

fn convert_string_to_symbol(s: &String, non_terms: &HashSet<String>, terms: &HashSet<String>) -> Result<Symbol, Box<dyn Error>> {
    if *s == EPSILON_SYMBOL.to_string() {
        return Ok(
            Symbol {
                kind: SymbolsKind::EPSILON,
                value: EPSILON_SYMBOL.to_string()
            }
        )
    }

    if non_terms.contains(s) {
        return Ok(
            Symbol {
                kind: SymbolsKind::NONTERM,
                value: s.to_string()
            }
        )
    }

    if terms.contains(s) {
        return Ok(
            Symbol {
                kind: SymbolsKind::TERM,
                value: s.to_string()
            }
        )
    }

    Err(ParseError::FailedConvertIntoSymbol.into())
}

pub fn write_json_to_file(g: &Grammar, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create(filename)?;
    let j = serde_json::to_string(g)?;
    file.write_all(j.as_bytes())?;

    Ok(())
}
