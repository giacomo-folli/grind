use clap::Parser;

use crate::{
    cli::{Args, Command},
    errors::TaskError,
    models::{DefaultState, Task},
};

mod cli;
mod commands;
mod errors;
mod models;
mod storage;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => storage::init()?,
        Command::List { status } => {
            let mut tasks = storage::load()?;

            if let Some(res) = status {
                tasks = tasks.iter().filter(|ts| ts.state == res).cloned().collect();
            }

            commands::list_tasks(&tasks)
        }
        Command::Add { title, description } => add_task(title, &description)?,
        Command::Edit {
            id,
            title,
            description,
        } => edit_task(id, &title, &description)?,
        Command::Status { id, status } => update_task_status(id, status)?,
        Command::Show { id } => show_task(id)?,
        Command::Delete { id } => delete_task(id)?,
    }

    Ok(())
}

fn show_task(task_id: String) -> anyhow::Result<()> {
    let tasks = storage::load()?;

    if let Some(found) = tasks.iter().find(|task| task.id == task_id) {
        println!("{}", toml::to_string(found)?);

        Ok(())
    } else {
        Err(TaskError::TaskNotFound(task_id).into())
    }
}

fn update_task_status(task_id: String, task_status: DefaultState) -> anyhow::Result<()> {
    let mut tasks = storage::load()?;

    if let Some(res) = tasks.iter_mut().find(|task| task.id == task_id) {
        res.state = task_status;
        res.update_time();

        storage::save(&tasks)?;

        Ok(())
    } else {
        Err(TaskError::TaskNotFound(task_id).into())
    }
}

fn add_task(task_title: Option<String>, task_description: &Option<String>) -> anyhow::Result<()> {
    let mut tasks = storage::load()?;
    let task_def_title;

    if let Some(res) = task_title {
        task_def_title = res;
    } else {
        task_def_title = "New task".to_string()
    }

    let mut new_task = Task::new(task_def_title);

    if let Some(desc) = task_description {
        new_task.description = Some(desc.clone());
    }

    tasks.push(new_task);

    storage::save(&tasks)?;

    Ok(())
}

fn edit_task(
    task_id: String,
    task_title: &Option<String>,
    task_description: &Option<String>,
) -> anyhow::Result<()> {
    let mut tasks = storage::load()?;

    if let Some(found) = tasks.iter_mut().find(|task| task.id == task_id) {
        if let Some(desc) = task_description {
            found.description = Some(desc.clone());
        }

        if let Some(title) = task_title {
            found.title = title.clone();
        }

        found.update_time(); // assuming always updating at least one field
        storage::save(&tasks)?;

        Ok(())
    } else {
        Err(TaskError::TaskNotFound(task_id).into())
    }
}

fn delete_task(task_id: String) -> anyhow::Result<()> {
    let mut tasks = storage::load()?;

    if let Some(found) = tasks.iter().find(|task| task.id == task_id) {
        tasks = tasks
            .iter()
            .filter(|task| task.id != found.id)
            .cloned()
            .collect();

        storage::save(&tasks)?;

        Ok(())
    } else {
        Err(TaskError::TaskNotFound(task_id).into())
    }
}
