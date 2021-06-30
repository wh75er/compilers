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
        grammar::Production::new_from_chars(vec![(NONTERM, 'A'), (NONTERM, 'B'), (NONTERM, 'C')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'A'), (TERM, 'a')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'B'), (NONTERM, 'C'), (NONTERM, 'A')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'B'), (NONTERM, 'A'), (TERM, 'b')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'C'), (NONTERM, 'A'), (NONTERM, 'B')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'C'), (NONTERM, 'C'), (NONTERM, 'C')]),
        grammar::Production::new_from_chars(vec![(NONTERM, 'C'), (TERM, 'a')]),
    ];

    let mut non_terms = HashSet::new();
    non_terms.insert('A');
    non_terms.insert('B');
    non_terms.insert('C');

    let mut terms = HashSet::new();
    terms.insert('a');
    terms.insert('b');

    let g = grammar::Grammar::new_from_chars(non_terms, terms, prods, 'A');

    // // let gm = transformations::remove_useless_symbols(&g);
    // let gm = transformations::to_e_free(&g);
    //
    // let gm2 = transformations::remove_useless_symbols(&gm);
    //
    // let gm3 = transformations::remove_unit_productions(&gm2);

    let gm3 = transformations::eliminate_indirect_lr(&g);

    println!("Init grammar {}", gm3);
}
