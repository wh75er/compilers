mod grammar;

use structopt::StructOpt;
use std::collections::HashSet;

use grammar::transformations;
use grammar::SymbolsKind::{TERM, NONTERM};
use crate::grammar::SymbolsKind::EPSILON;

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
        grammar::Production::new(&vec!(
            (NONTERM, "S"), (TERM, "a"), (NONTERM, "A"), (NONTERM, "B")
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "S"), (NONTERM, "C")
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "D"), (TERM, "c"), (NONTERM, "D"), (TERM, "c")
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "D"), (TERM, "d")
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "C"), (TERM, "a"), (NONTERM, "C"), (NONTERM, "D")
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "A"), (TERM, "a"), (NONTERM, "A")
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "A"), (TERM, "a")
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "A"), (EPSILON, grammar::EPSILON_SYMBOL)
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "B"), (TERM, "b")
        )),
    );

    let mut non_terms = HashSet::new();
    non_terms.insert("S");
    non_terms.insert("A");
    non_terms.insert("B");
    non_terms.insert("C");
    non_terms.insert("D");

    let mut terms = HashSet::new();
    terms.insert("a");
    terms.insert("b");
    terms.insert("c");
    terms.insert("d");
    terms.insert("e");

    let g = grammar::Grammar::new(&non_terms, &terms, &prods, "S");

    let gm = transformations::remove_useless_symbols(&g);

    println!("Init grammar {:?}", gm);
}
