mod grammar;

use structopt::StructOpt;
use std::collections::HashSet;

use grammar::SymbolsKind::{TERM, NONTERM};

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
            (NONTERM, "S".to_string()), (NONTERM, "E".to_string()), (TERM, "+".to_string()), (NONTERM, "T".to_string()),
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "E".to_string()), (NONTERM, "T".to_string()),
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "T".to_string()), (NONTERM, "T".to_string()), (TERM, "*".to_string()), (NONTERM, "F".to_string()),
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "T".to_string()), (NONTERM, "F".to_string()),
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "F".to_string()), (TERM, "a".to_string()),
        )),
        grammar::Production::new(&vec!(
            (NONTERM, "F".to_string()), (TERM, "(".to_string()), (NONTERM, "E".to_string()), (TERM, ")".to_string()),
        )),
    );

    let mut non_terms = HashSet::new();
    non_terms.insert(String::from('E'));
    non_terms.insert(String::from('T'));
    non_terms.insert(String::from('F'));

    let mut terms = HashSet::new();
    terms.insert(String::from('+'));
    terms.insert(String::from('*'));
    terms.insert(String::from('('));
    terms.insert(String::from(')'));
    terms.insert(String::from('a'));

    let g = grammar::Grammar::new(&non_terms, &terms, &prods, "E");

    println!("Init grammar {:?}", g);
}
