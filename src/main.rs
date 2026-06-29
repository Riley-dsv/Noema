use clap::Parser;

use crate::{cli::Cli, database::sqlite::SQLStore};

mod cli;
mod commands;
mod database;
mod editor;
mod error;
mod path;

fn main() {
    if let Err(err) = run() {
        eprintln!("noema encountered an error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let db_path = cli.path.unwrap_or_else(path::default_database_path);
    let store = SQLStore::open(db_path)?;
    store.init()?;
    store.migrate()?;

    match cli.command {
        cli::Command::Init => store.init()?,
        cli::Command::Note { command } => match command {
            cli::NoteCommand::Search { keyword, tag } => {
                commands::search::search_in_notes(&store, keyword.as_deref(), tag.as_deref())?
            }
            cli::NoteCommand::Create { title, content } => {
                commands::create::create_note(&store, &title, content.as_deref())?
            }
            cli::NoteCommand::List => commands::list::list_notes(&store)?,
            cli::NoteCommand::Read { id } => commands::read::read_note(&store, &id)?,
            cli::NoteCommand::Info { id } => commands::info::note_info(&store, &id)?,
            cli::NoteCommand::Update { id, title } => {
                commands::update::update_note(&store, &id, title.as_deref())?
            }
            cli::NoteCommand::Delete { id, tag } => {
                if let Some(tag) = tag {
                    commands::delete::detach_tag_from_note(&store, &id, &tag)?;
                    return Ok(());
                }
                commands::delete::delete_note(&store, &id)?;
            }
        },
        cli::Command::Tag { tag } => match tag {
            cli::TagCommand::List => commands::list::list_tags(&store)?,
            cli::TagCommand::Create {
                tag,
                attach: note_id,
            } => {
                commands::create::create_tag(&store, &tag)?;
                if let Some(note_id) = note_id {
                    commands::update::attach_tag_to_note(&store, &note_id, &tag)?;
                }
            }
            cli::TagCommand::Delete { tag } => commands::delete::delete_tag(&store, &tag)?,
            cli::TagCommand::Attach { tag, note } => {
                commands::update::attach_tag_to_note(&store, &note, &tag)?
            }
        },
    }
    Ok(())
}
