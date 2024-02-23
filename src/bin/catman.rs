#[path = "catman/mod.rs"]
mod commands;

use commands::{add_command, delete_command, entries_command, list_command, show_command};

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
    Add(AddArgs),
    List(ListArgs),
    Show(NameArg),
    Entries(NameArg),
    Delete(NameArg),
}

#[derive(Args, Debug, Clone)]
struct AddArgs {
    name: String,
    disp_name: Option<String>,
    #[arg(default_value_t = 20)]
    num_entries: i32,
}

#[derive(Args, Debug, Clone)]
struct ListArgs {
    #[arg(short, long)]
    details: bool,
}

#[derive(Args, Debug, Clone)]
struct NameArg {
    name: String,
}

fn main() -> Result<(), String> {
    let app = App::parse();

    let res = match &app.command {
        Commands::Add(args) => add_command(args)?,
        Commands::List(args) => list_command(args)?,
        Commands::Show(args) => show_command(args)?,
        Commands::Entries(args) => entries_command(args)?,
        Commands::Delete(args) => delete_command(args)?,
    };

    Ok(res)
}
