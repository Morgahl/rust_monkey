mod repl;

use anyhow::Result;
use clap::{Parser, Subcommand};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Repl) => cmd_repl(),
        None => cmd_repl(),
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run the REPL (default command)")]
    Repl,
}

fn cmd_repl() -> Result<()> {
    let stdin = std::io::stdin();
    let stdoout = std::io::stdout();
    repl::start(&mut stdin.lock(), &mut stdoout.lock())
}
