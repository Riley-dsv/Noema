use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct SearchArgs {
    #[arg(required_unless_present = "tags")]
    pub keyword: Option<String>,
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub tags: Option<Vec<String>>,
    #[arg(long, requires = "tags")]
    pub match_any: bool,
    #[arg(long, requires = "keyword")]
    pub title: bool,
    #[arg(long, requires = "keyword")]
    pub content: bool,
    #[arg(short, long)]
    pub limit: Option<usize>,
}

#[derive(Subcommand)]
pub enum NoteCommand {
    Search {
        #[arg(required_unless_present = "tag")]
        keyword: Option<String>,
        #[arg(long)]
        tag: Option<String>,
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
        #[arg(long)]
        tag: Option<String>,
        id: String,
    },
}

#[derive(Subcommand)]
pub enum TagCommand {
    List,
    Create {
        tag: String,
        #[arg(long)]
        attach: Option<String>,
    },
    Delete {
        tag: String,
    },
    Attach {
        tag: String,
        note: String,
    },
    Detach {
        tag: String,
        note: String,
    },
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    Note {
        #[command(subcommand)]
        command: NoteCommand,
    },
    Tag {
        #[command(subcommand)]
        tag: TagCommand,
    },
    Search(SearchArgs),
}

#[derive(Parser)]
#[command(name = "noema")]
#[command(about = "A native personal knowledge base")]
pub struct Cli {
    #[arg(long, global = true)]
    pub path: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Command,
}
