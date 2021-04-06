mod fsm;
mod syntax_tree;
mod draw;

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

    dfa.render_to("dfa.dot");

    let minimized_dfa = fsm::dfa_minimization::minimize(&dfa);

    println!("Minimized dfa: {:#?}", minimized_dfa);

    minimized_dfa.render_to("min_dfa.dot");
}
