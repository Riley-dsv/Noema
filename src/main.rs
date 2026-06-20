use clap::{Parser, Subcommand};
use std::{env, path::PathBuf};

use crate::database::sqlite::SQLStore;

mod commands;
mod database;
mod editor;
mod error;
mod path;

#[derive(Subcommand)]
enum NoteCommand {
    Search {
        keyword: String,
    },
    Create {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: Option<String>,
    },
    Info {
        id: String,
    },
    List,
    Read {
        id: String,
    },
    Update {
        id: String,
        #[arg(short, long)]
        title: Option<String>,
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
    #[arg(long, global = true)]
    path: Option<PathBuf>,
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
    let db_path = cli.path.unwrap_or_else(path::default_database_path);
    let store = SQLStore::open(db_path)?;
    store.migrate()?;

    match cli.command {
        Command::Init => store.init()?,
        Command::Note { command } => match command {
            NoteCommand::Search { keyword } => commands::search::search_in_notes(&store, &keyword)?,
            NoteCommand::Create { title, content } => {
                commands::create::create_note(&store, &title, content.as_deref())?
            }
            NoteCommand::List => commands::list::list_notes(&store)?,
            NoteCommand::Read { id } => commands::read::read_note(&store, &id)?,
            NoteCommand::Info { id } => commands::info::note_info(&store, &id)?,
            NoteCommand::Update { id, title } => {
                commands::update::update_note(&store, &id, title.as_deref())?
            }
            NoteCommand::Delete { id } => commands::delete::delete_note(&store, &id)?,
        },
    }
    Ok(())
}
