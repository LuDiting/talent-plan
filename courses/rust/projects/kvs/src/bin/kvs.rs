use std::{env::current_dir, process::exit};

use clap::{Parser, Subcommand};
use kvs::KvStore;
use serde::de::value;

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
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
    Ls,
}

fn main() {
    let mut service = KvStore::open(current_dir().unwrap().as_path()).unwrap();
    let cli = Cli::parse();
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }
    execute(&mut service, cli.command);
}

fn execute(service: &mut KvStore, cmd: Option<Commands>) {
    match cmd {
        Some(Commands::Set { key, value }) => match service.set(key, value) {
            Ok(_r) => {}
            Err(e) => {
                println!("{}", e);
                exit(1)
            }
        },
        Some(Commands::Get { key }) => match service.get(key) {
            Ok(r) => {
                match r {
                    Some(value) => {println!("{}", value);}
                    None => {println!("Key not found");}
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        },
        Some(Commands::Rm { key }) => match service.remove(key) {
            Ok(_r) => {}
            Err(e) => {
                println!("{}", e);
                exit(1)
            }
        },
        Some(Commands::Ls) => {
            service.ls();
        }
        None => {}
    }
}
