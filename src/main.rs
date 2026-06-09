use clap::{Parser, Subcommand};
use std::{env, path::PathBuf};

mod commands;
mod database;
mod editor;
mod error;

#[derive(Subcommand)]
enum NoteCommand {
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
    Init {
        path: Option<PathBuf>,
    },
    Note {
        #[arg(long)]
        db: Option<PathBuf>,
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
        Command::Init { path } => commands::init::init_database(path)?,
        Command::Note { db, command } => match command {
            NoteCommand::Create { title, content } => commands::create::create_note(db, title, content)?,
            NoteCommand::List => commands::list::list_notes(db)?,
            NoteCommand::Read { id } => commands::read::read_note(db, &id)?,
            NoteCommand::Info { id } => commands::info::note_info(db, &id)?,
            NoteCommand::Update { id, title } => commands::update::update_note(db, &id, title)?,
            NoteCommand::Delete { id } => commands::delete::delete_note(db, &id)?,
        },
    }
    Ok(())
}
