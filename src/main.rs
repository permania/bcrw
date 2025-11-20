mod cli;
mod data;

use data::buns;

fn main() {
    // test stuff
    println!("{}", buns::BUNS.get("shy").unwrap());
}
