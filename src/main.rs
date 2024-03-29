mod cmd;
mod config;
mod core;
mod git;

use anyhow::Result;
use clap::Parser;
use cmd::Commands;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gdpack", author, version, about)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// A `PATH` to the Godot project containing the manifest.
    #[arg(short, long, global = true, value_name = "PATH")]
    project: Option<PathBuf>,

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
        Commands::Add(args) => cmd::add::handle(cli.project, args),
        Commands::Remove(args) => cmd::remove::handle(cli.project, args),
        Commands::Replace(args) => cmd::replace::handle(cli.project, args),

        /* ------------------------- Category: Init ------------------------- */
        Commands::Init => cmd::init::handle(cli.project),

        /* ------------------------ Category: Install ----------------------- */
        Commands::Install(args) => cmd::install::handle(cli.project, args),
    }
}
