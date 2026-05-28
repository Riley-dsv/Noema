use clap::{Parser, Subcommand};
use std::env;

use crate::{database::sqlite, editor::open_editor};

mod database;
mod editor;

#[derive(Subcommand)]
enum NoteCommand {
    Create {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: Option<String>,
    },
    List,
    Read {
        id: String,
    },
    Update {
        id: String,
    },
    Delete {
        id: String,
    },
}

#[derive(Subcommand)]
enum Command {
    Init,
    Note {
        #[command(subcommand)]
        command: NoteCommand,
    },
}

#[derive(Parser)]
#[command(name = "noema")]
#[command(about = "A native personal knowledge base")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("noema encountered an error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    env::args().nth(1).expect("No command given");

    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            sqlite::init()?;
        }
        Command::Note { command } => match command {
            NoteCommand::Create { title, content } => {
                let editor_content = open_editor(&content.unwrap_or_default())?;
                sqlite::insert(title, editor_content)?;
            }
            NoteCommand::List => {
                sqlite::list()?;
            }
            NoteCommand::Read { id } => {
                let content = sqlite::get_content(&id)?;
                println!("{content}");
            }
            NoteCommand::Update { id } => {
                let old_content = sqlite::get_content(&id)?;
                let new_content = open_editor(&old_content)?;
                sqlite::update(&new_content, id)?;
            }
            NoteCommand::Delete { id } => {
                sqlite::delete(id)?;
            }
        },
    }
    Ok(())
}
