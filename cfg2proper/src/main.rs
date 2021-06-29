#![feature(drain_filter)]
mod grammar;

#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
use structopt::StructOpt;

use crate::grammar::SymbolsKind::EPSILON;
use crate::grammar::EPSILON_SYMBOL;
use grammar::transformations;
use grammar::SymbolsKind::{NONTERM, TERM};

#[derive(Debug, StructOpt)]
#[structopt(name = "cfg2proper", about = "This utility converts CFG to proper CFG")]
struct Opt {
    /// Input file with CFG in JSON format
    #[structopt(default_value = "input_cfg.json")]
    filename: String,
}

fn main() {
    let opt = Opt::from_args();

    // let prods: Vec<grammar::Production> = vec![
    //     grammar::Production::new_from_chars(vec![(NONTERM, 'S'), (NONTERM, 'B'), (NONTERM, 'C')]),
    //     grammar::Production::new_from_chars(vec![(NONTERM, 'S'), (NONTERM, 'A'), (TERM, 'b')]),
    //     grammar::Production::new_from_chars(vec![(NONTERM, 'B'), (EPSILON, EPSILON_SYMBOL)]),
    //     grammar::Production::new_from_chars(vec![(NONTERM, 'C'), (TERM, 'c')]),
    //     grammar::Production::new_from_chars(vec![(NONTERM, 'A'), (NONTERM, 'A'), (TERM, 'a')]),
    //     grammar::Production::new_from_chars(vec![(NONTERM, 'A'), (EPSILON, EPSILON_SYMBOL)]),
    // ];
    //
    // let mut non_terms = HashSet::new();
    // non_terms.insert('S');
    // non_terms.insert('A');
    // non_terms.insert('B');
    // non_terms.insert('C');
    //
    // let mut terms = HashSet::new();
    // terms.insert('a');
    // terms.insert('b');
    // terms.insert('c');

    let prods: Vec<grammar::Production> = vec![
        grammar::Production::new_from_chars(vec![(NONTERM, 'E'), (NONTERM, 'E'), (TERM, '+'), (NONTERM, 'T')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'E'), (NONTERM, 'T')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'T'), (NONTERM, 'T'), (TERM, '*'), (NONTERM, 'F')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'T'), (NONTERM, 'F')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'F'), (TERM, '('), (NONTERM, 'E'), (TERM, ')')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'F'), (TERM, 'a')]),
    ];

    let mut non_terms = HashSet::new();
    non_terms.insert('E');
    non_terms.insert('T');
    non_terms.insert('F');

    let mut terms = HashSet::new();
    terms.insert('a');
    terms.insert('(');
    terms.insert(')');

    let g = grammar::Grammar::new_from_chars(non_terms, terms, prods, 'S');

    // // let gm = transformations::remove_useless_symbols(&g);
    // let gm = transformations::to_e_free(&g);
    //
    // let gm2 = transformations::remove_useless_symbols(&gm);
    //
    // let gm3 = transformations::remove_unit_productions(&gm2);

    let mut new_non_terms = HashSet::new();
    new_non_terms.insert(String::from('E'));
    new_non_terms.insert(String::from('T'));
    new_non_terms.insert(String::from('F'));
    let mut new_terms = HashSet::new();
    new_terms.insert(String::from('a'));
    new_terms.insert(String::from('('));
    new_terms.insert(String::from(')'));

    let mut fixed_e_prods = g.productions.iter().cloned().fold(vec![], |mut acc, prod| {
        if prod.replaced_symbol.value == 'E'.to_string() {
            acc.push(prod);
        }

        acc
    });
    let new_non_term = transformations::eliminate_immediate_lr(&mut fixed_e_prods);
    new_non_terms.insert(new_non_term.unwrap());

    let mut new_prods = vec![];
    new_prods.extend_from_slice(&fixed_e_prods);

    let mut fixed_t_prods = g.productions.iter().cloned().fold(vec![], |mut acc, prod| {
        if prod.replaced_symbol.value == 'T'.to_string() {
            acc.push(prod);
        }

        acc
    });
    let new_non_term = transformations::eliminate_immediate_lr(&mut fixed_t_prods);
    new_non_terms.insert(new_non_term.unwrap());

    new_prods.extend_from_slice(&fixed_t_prods);

    new_prods.extend_from_slice( &vec![
            grammar::Production::new_from_chars(vec![(NONTERM, 'F'), (TERM, '('), (NONTERM, 'E'), (TERM, ')')]),
            grammar::Production::new_from_chars(vec![(NONTERM, 'F'), (TERM, 'a')]),
        ][..]
    );

    let gm3 = grammar::Grammar::new(new_non_terms, new_terms, new_prods.iter().cloned().collect(), String::from("E"));

    println!("Init grammar {}", gm3);
}
