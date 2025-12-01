use clap::{
    Args, Parser, Subcommand,
    builder::{Styles, styling::AnsiColor},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, styles=STYLES)]
pub struct BunArgs {
    #[clap(subcommand)]
    pub cmd: BunCommand,
}

#[derive(Debug, Subcommand)]
pub enum BunCommand {
    /// print a specific bunny
    #[command()]
    Show(ShowBun),

    /// print a random bunny
    Rand(RandArgs),

    /// list available bunnies
    List(ListArgs),
}

#[derive(Debug, Args)]
pub struct ShowBun {
    #[arg(default_value = "normal")]
    pub bunny: String,
}

#[derive(Debug, Args)]
pub struct RandArgs {
    /// omit the name of the bunny
    #[arg(short = 'n', long, default_value_t = false)]
    pub hide_name: bool,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// show the bunnies with their names
    #[arg(short = 's', long, default_value_t = false)]
    pub show: bool,
}

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::BrightCyan.on_default())
    .placeholder(AnsiColor::BrightWhite.on_default());
