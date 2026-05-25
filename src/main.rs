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
    env::args().nth(1).expect("No command given");

    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            let _ = sqlite::init();
        }
        Command::Note { command } => match command {
            NoteCommand::Create { title } => {
                let content = open_editor("").expect("Failed to open an editor");
                let _ = sqlite::insert(title, content);
            }
            NoteCommand::List => {
                let _ = sqlite::list();
            }
            NoteCommand::Read { id } => {
                let content = sqlite::get_content(&id);
                open_editor(&content.unwrap()).expect("Failed to open an editor");
            }
            NoteCommand::Update { id } => {
                let old_content = sqlite::get_content(&id);
                let new_content =
                    open_editor(&old_content.unwrap()).expect("Failed to open an editor");
                let _ = sqlite::update(&new_content, id);
            }
            NoteCommand::Delete { id } => {
                let _ = sqlite::drop(id);
            }
        },
    }
}
