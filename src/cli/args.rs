use clap::{
    Parser,
    builder::{Styles, styling::AnsiColor},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, styles=STYLES)]
pub struct BunArgs {
    /// which bunny to print
    #[clap(short)]
    pub bunny: String,
}

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::BrightCyan.on_default())
    .placeholder(AnsiColor::BrightWhite.on_default());
