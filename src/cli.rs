use clap::{Parser, Subcommand};

use crate::models::DefaultStatus;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create the todo.toml status file
    Init,
    /// View the current status
    List {
        #[arg(short, long)]
        status: Option<DefaultStatus>,
    },
    /// Edit a task
    Edit {
        id:          String,
        #[arg(short, long, required_unless_present = "title")]
        description: Option<String>,
        #[arg(short, long, required_unless_present = "description")]
        title:       Option<String>,
    },
    /// Add a new task in #todo
    Add {
        title:       Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Change a task's status
    Status {
        id:     String,
        status: DefaultStatus,
    },
    /// Delete a task
    Delete { id: String },
}

/// Simple task managment cli tool
#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[command(subcommand)]
    pub command: Command,
}
