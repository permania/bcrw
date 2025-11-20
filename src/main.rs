mod cli;
mod data;

use data::buns;

fn main() {
    // test stuff
    println!("{}", buns::MAP.get("shocked").unwrap());
}
