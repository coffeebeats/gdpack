mod addon;
mod cmd;
mod config;
mod git;

use anyhow::Result;
use clap::Parser;
use cmd::Commands;

#[derive(Parser)]
#[command(name = "gdpack", author, version, about)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Silences all non-essential logging.
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    quiet: bool,

    /// Enables additional detailed logging.
    #[arg(short, long, global = true)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        /* ----------------------- Category: Dependencies ---------------------- */
        Commands::Add(args) => cmd::add::handle(args),
        Commands::Remove(args) => cmd::remove::handle(args),
        Commands::Replace(args) => cmd::replace::handle(args),

        /* ------------------------- Category: Init ------------------------- */
        Commands::Init(args) => cmd::init::handle(args),

        /* ------------------------ Category: Install ----------------------- */
        Commands::Install(args) => cmd::install::handle(args),
    }
}
