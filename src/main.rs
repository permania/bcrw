mod cli;
mod data;

use clap::Parser;
use cli::args::BunCommand::{List, Rand, Show};
use cli::args::{BunArgs, ShowBun};
use cli::error::handle_error;
use data::buns;

fn main() {
    let cli = BunArgs::parse();

    match cli.cmd {
        Show(ShowBun { bunny: bun }) => handle_error(buns::print_bun(&bun)),
        Rand(a) => buns::random_bun(a),
        List(a) => buns::list_buns(a),
    }
}
