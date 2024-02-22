use diesel_demo::{
    connect::establish_connection,
    repository::category::{
        all_categories, category_by_name, category_entries, create_category,
        next_parent_enum_id_for, num_category_entries,
    },
};

use std::{path::PathBuf, process::ExitCode};

#[path = "catman/mod.rs"]
mod commands;

use commands::add_command;

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
}

#[derive(Args, Debug, Clone)]
struct AddArgs {
    name: String,
    disp_name: Option<String>,
    #[arg(default_value_t = 20)]
    num_entries: i32,
}

fn main() -> Result<(), String> {
    let app = App::parse();

    println!("Let's start");

    match &app.command {
        Commands::Add(args) => Ok(add_command(args)?),
    }
}
