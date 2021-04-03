mod fsm;
mod syntax_tree;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "regex2fsm",
    about = "This utility converts basic regex expression to deterministic finite automaton"
)]
struct Opt {
    ///
    #[structopt()]
    regex: String,
}

fn main() {
    let opt = Opt::from_args();

    let result = syntax_tree::parser::parse(&opt.regex);

    let dfa = fsm::dfa::transform(result.unwrap());

    println!("Dfa: {:#?}", dfa);
}
