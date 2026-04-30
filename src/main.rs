use clap::Parser;

use crate::{
    cli::{Args, Command},
    service::TaskService,
};

mod cli;
mod errors;
mod models;
mod output;
mod service;
mod storage;
mod tests;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let service = TaskService::new();

    match args.command {
        Command::Init => storage::init()?,
        Command::Add { title, description } => {
            service.add_task(title, description)?
        },
        Command::Edit {
            id,
            title,
            description,
        } => service.edit_task(&id, title, description)?,
        Command::Status { id, status } => service.update_status(&id, status)?,
        Command::Delete { id } => service.delete_task(&id)?,
        Command::List { status } => {
            let tasks = service.list_tasks(status)?;
            output::display_tasks(&tasks);
        },
    }

    Ok(())
}
