#![feature(drain_filter)]
mod grammar;

#[macro_use]
extern crate lazy_static;

use structopt::StructOpt;

use grammar::transformations;
use grammar::parser::parse_from_file;

#[derive(Debug, StructOpt)]
#[structopt(name = "cfg2proper", about = "This utility converts CFG to proper CFG")]
struct Opt {
    /// Input file with CFG in JSON format
    #[structopt(default_value = "input_cfg.txt")]
    filename: String,
}

fn main() {
    let opt = Opt::from_args();

    let g = parse_from_file(&opt.filename);
    let g = match g {
        Ok(v) => v,
        Err(e) => {
            panic!("Failed to parse cfg from file: {}", e);
        }
    };

    let gm1 = transformations::to_e_free(&g);

    let gm2 = transformations::remove_useless_symbols(&gm1);

    let gm3 = transformations::remove_unit_productions(&gm2);

    let gm4 = transformations::eliminate_indirect_lr(&gm3);

    println!("Init grammar {}", gm4);
}
