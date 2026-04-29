use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::Context;

use crate::models::{Storage, Task};

pub const FILE: &str = "tasks.toml";

pub fn init() -> anyhow::Result<()> {
    let _ = File::create(FILE)?;

    Ok(())
}

pub fn load() -> anyhow::Result<Vec<Task>> {
    if !std::path::Path::new(FILE).exists() {
        init()?
    }

    let raw = fs::read_to_string(FILE).context("Failed to read tasks file")?;
    let parsed: Storage = toml::from_str(&raw).context("Failed to parse tasks file")?;

    Ok(parsed.tasks)
}

pub fn save(tasks: &[Task]) -> anyhow::Result<()> {
    let storage = Storage {
        tasks: tasks.to_owned(),
    };

    let res = toml::to_string(&storage).context("Failed to serialize tasks")?;
    let mut file = File::create(FILE).context("Failed to create tasks file")?;

    file.write_all(res.as_bytes())
        .context("Failed to write tasks file")?;

    Ok(())
}
