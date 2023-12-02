mod cmd;

use anyhow::Result;
use clap::Parser;
use cmd::Commands;

#[derive(Parser)]
#[command(name = "gvm", author, version, about, verbatim_doc_comment)]
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
        Commands::Add(args) => cmd::handle_add(args),
    }
}
