use std::sync::Arc;

use crate::{
    errors::TaskError,
    models::{DefaultStatus, Task},
    storage::{FileStorage, StorageBackend},
};

pub struct TaskService {
    storage: Arc<dyn StorageBackend>,
}

impl TaskService {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(FileStorage),
        }
    }

    #[allow(dead_code)]
    pub fn with_storage(storage: Arc<dyn StorageBackend>) -> Self {
        Self { storage }
    }

    pub fn list_tasks(
        &self,
        filter: Option<DefaultStatus>,
    ) -> anyhow::Result<Vec<Task>> {
        let tasks = self.storage.load()?;
        Ok(match filter {
            Some(status) => tasks
                .iter()
                .filter(|t| t.status == status)
                .cloned()
                .collect(),
            None => tasks,
        })
    }

    pub fn add_task(
        &self,
        title: Option<String>,
        description: Option<String>,
    ) -> anyhow::Result<()> {
        let mut tasks = self.storage.load()?;
        let title = title.unwrap_or_else(|| "New task".to_string());
        let mut new_task = Task::new(title);
        new_task.description = description;
        tasks.push(new_task);
        self.storage.save(&tasks)
    }

    pub fn edit_task(
        &self,
        task_id: &str,
        title: Option<String>,
        description: Option<String>,
    ) -> anyhow::Result<()> {
        let mut tasks = self.storage.load()?;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            if let Some(desc) = description {
                task.description = Some(desc);
            }
            if let Some(new_title) = title {
                task.title = new_title;
            }
            task.update_time();
            self.storage.save(&tasks)
        } else {
            Err(TaskError::TaskNotFound(task_id.to_string()).into())
        }
    }

    pub fn update_status(
        &self,
        task_id: &str,
        status: DefaultStatus,
    ) -> anyhow::Result<()> {
        let mut tasks = self.storage.load()?;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = status;
            task.update_time();
            self.storage.save(&tasks)
        } else {
            Err(TaskError::TaskNotFound(task_id.to_string()).into())
        }
    }

    pub fn delete_task(&self, task_id: &str) -> anyhow::Result<()> {
        let mut tasks = self.storage.load()?;
        if tasks.iter().any(|t| t.id == task_id) {
            tasks.retain(|t| t.id != task_id);
            self.storage.save(&tasks)
        } else {
            Err(TaskError::TaskNotFound(task_id.to_string()).into())
        }
    }
}

impl Default for TaskService {
    fn default() -> Self {
        Self::new()
    }
}
