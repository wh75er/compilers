mod grammar;

use structopt::StructOpt;
use std::collections::HashSet;

use grammar::transformations;
use grammar::SymbolsKind::{TERM, NONTERM};
use crate::grammar::SymbolsKind::EPSILON;
use crate::grammar::EPSILON_SYMBOL;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cfg2proper",
    about = "This utility converts CFG to proper CFG"
)]
struct Opt {
    /// Input file with CFG in JSON format
    #[structopt(default_value="input_cfg.json")]
    filename: String,
}

fn main() {
    let opt = Opt::from_args();

    let prods: Vec<grammar::Production> = vec!(
        grammar::Production::new(vec!(
            (NONTERM, 'S'), (NONTERM, 'B'), (NONTERM, 'C'),
        )),
        grammar::Production::new(vec!(
            (NONTERM, 'S'), (NONTERM, 'A'), (TERM, 'b'),
        )),
        grammar::Production::new(vec!(
            (NONTERM, 'B'), (EPSILON, EPSILON_SYMBOL),
        )),
        grammar::Production::new(vec!(
            (NONTERM, 'C'), (TERM, 'c'),
        )),
        grammar::Production::new(vec!(
            (NONTERM, 'A'), (NONTERM, 'A'), (TERM, 'a')
        )),
        grammar::Production::new(vec!(
            (NONTERM, 'A'), (EPSILON, EPSILON_SYMBOL),
        )),
    );

    let mut non_terms = HashSet::new();
    non_terms.insert('S');
    non_terms.insert('A');
    non_terms.insert('B');
    non_terms.insert('C');

    let mut terms = HashSet::new();
    terms.insert('a');
    terms.insert('b');
    terms.insert('c');

    let g = grammar::Grammar::new(non_terms, terms, prods, 'S');

    // let gm = transformations::remove_useless_symbols(&g);
    let gm = transformations::to_e_free(&g);

    let gm2 = transformations::remove_useless_symbols(&gm);

    let gm3 = transformations::remove_unit_productions(&gm2);

    println!("Init grammar {:?}", gm3);
}
