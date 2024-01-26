use std::process::exit;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(arg_required_else_help(true))]
struct Cli {
    name: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Set{
        key :String,
        value :String,
    },
    Get{
        key :String,
    },
    Rm{
        key :String,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Set { key, value }) => {
            eprintln!("unimplemented");
            exit(1);
        },
        Some(Commands::Get { key }) => {
            eprintln!("unimplemented");
            exit(1);
        },
        Some(Commands::Rm { key }) => {
            eprintln!("unimplemented");
            exit(1);
        }
        None => {}
    }

    // Continued program logic goes here...
}