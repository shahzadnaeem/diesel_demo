#[path = "catman2/mod.rs"]
mod commands;

use crate::commands::test_command;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version,about,long_about = None)]
#[command(propagate_version = true)]
struct App {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Test(TestArgs),
}

#[derive(Args, Debug, Clone)]
struct TestArgs {
    name: String,
    alt_name: Option<String>,
    #[arg(short, long, default_value_t = 20)]
    num_entries: i32,
}

type AppResult = anyhow::Result<()>;

fn main() -> AppResult {
    let app = App::parse();

    let res = match &app.command {
        Commands::Test(args) => test_command(args)?,
    };

    Ok(res)
}
