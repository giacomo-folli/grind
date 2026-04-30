use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Context;
use dirs::config_dir;

use crate::models::{Storage as StorageModel, Task};

pub const FILE_NAME: &str = "tasks.toml";

pub trait StorageBackend {
    fn load(&self) -> anyhow::Result<Vec<Task>>;
    fn save(&self, tasks: &[Task]) -> anyhow::Result<()>;
}

pub struct FileStorage;

impl FileStorage {
    fn get_config() -> anyhow::Result<PathBuf> {
        let mut config_path =
            config_dir().context("Could not find config directory")?;
        config_path.push("grind");

        if !config_path.exists() {
            fs::create_dir_all(&config_path)
                .context("Failed to create config directory")?;
        }

        Ok(config_path.join(FILE_NAME))
    }
}

impl StorageBackend for FileStorage {
    fn load(&self) -> anyhow::Result<Vec<Task>> {
        let file_path = Self::get_config()?;
        let raw = fs::read_to_string(&file_path)
            .context("Failed to read tasks file")?;
        let parsed: StorageModel =
            toml::from_str(&raw).context("Failed to parse tasks file")?;
        Ok(parsed.tasks)
    }

    fn save(&self, tasks: &[Task]) -> anyhow::Result<()> {
        let file_path = Self::get_config()?;
        let storage = StorageModel {
            tasks: tasks.to_owned(),
        };
        let res =
            toml::to_string(&storage).context("Failed to serialize tasks")?;
        let mut file =
            File::create(file_path).context("Failed to create tasks file")?;
        file.write_all(res.as_bytes())
            .context("Failed to write tasks file")?;
        Ok(())
    }
}

pub fn init() -> anyhow::Result<()> {
    let file_path = FileStorage::get_config()?;
    if !file_path.exists() {
        File::create(&file_path).context("Failed to create tasks file")?;
    }
    Ok(())
}
