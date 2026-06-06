use clap::{Parser, Subcommand};
use std::{env, io, path::PathBuf};

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
        Command::Init { path } => {
            sqlite::init(path)?;
        }
        Command::Note { db, command } => match command {
            NoteCommand::Create { title, content } => {
                let editor_content = open_editor(&content.unwrap_or_default())?;
                sqlite::insert(db, title, editor_content)?;
            }
            NoteCommand::List => {
                sqlite::list(db)?;
            }
            NoteCommand::Read { id } => {
                let content = sqlite::get_content(db, &id)?;
                println!("{content}");
            }
            NoteCommand::Info { id } => {
                let note = sqlite::select(db.clone(), id.clone())?;
                let content_size = String::from(sqlite::get_content(db, &id)?).len();
                println!(
                    "ID: {}\nTitle: {}\nCreated At: {}\nLast Updated: {}\nSize: {}Bytes",
                    &id, note.title, note.created_at, note.updated_at, content_size
                );
            }
            NoteCommand::Update { id, title } => {
                if let Some(title) = title {
                    sqlite::update_title(db.clone(), &id, title)?;
                } else {
                    let old_content = sqlite::get_content(db.clone(), &id)?;
                    let new_content = open_editor(&old_content)?;
                    sqlite::update(db, &new_content, id)?;
                }
            }
            NoteCommand::Delete { id } => {
                print!("Are you sure you want to delete note: {} (Y/N) > ", id);

                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Unable to read stdin");

                if input.trim().to_lowercase() == "y" {
                    sqlite::delete(db, id)?;
                }
            }
        },
    }
    Ok(())
}
