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
        // `ShowBun` sets the value of `bun` to "normal" if none is specified, `bun show` is equivalent to `bun show normal`.
        Show(ShowBun { bunny: bun }) => handle_error(buns::print_bun(&bun)),
        Rand(a) => buns::random_bun(a),
        List(a) => buns::list_buns(a),
    }
}
