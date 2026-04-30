use std::sync::{Arc, RwLock};

use super::*;
use crate::{
    models::{DefaultStatus, Task},
    storage::StorageBackend,
};

#[allow(dead_code)]
pub struct InMemoryStorage {
    data: RwLock<Vec<Task>>,
}

#[allow(dead_code)]
impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(Vec::new()),
        }
    }

    pub fn with_tasks(tasks: Vec<Task>) -> Self {
        Self {
            data: RwLock::new(tasks),
        }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageBackend for InMemoryStorage {
    fn load(&self) -> anyhow::Result<Vec<Task>> {
        Ok(self.data.read().unwrap().clone())
    }

    fn save(&self, tasks: &[Task]) -> anyhow::Result<()> {
        let mut data = self.data.write().unwrap();
        *data = tasks.to_vec();
        Ok(())
    }
}

#[allow(dead_code)]
fn test_service() -> TaskService {
    TaskService::with_storage(Arc::new(InMemoryStorage::new()))
}

#[allow(dead_code)]
fn test_service_with_tasks(tasks: Vec<Task>) -> TaskService {
    TaskService::with_storage(Arc::new(InMemoryStorage::with_tasks(tasks)))
}

#[allow(dead_code)]
fn make_task(id: &str, title: &str, status: DefaultStatus) -> Task {
    Task {
        id: id.to_string(),
        title: title.to_string(),
        description: None,
        status,
        created_at: "Thu, 30 Apr 2026 00:00:00 +0000".to_string(),
        updated_at: "Thu, 30 Apr 2026 00:00:00 +0000".to_string(),
    }
}

#[test]
fn test_add_task_with_title() {
    let service = test_service();
    service
        .add_task(Some("Test task".to_string()), None)
        .unwrap();
    let tasks = service.list_tasks(None).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title, "Test task");
    assert_eq!(tasks[0].status, DefaultStatus::Todo);
}

#[test]
fn test_add_task_default_title() {
    let service = test_service();
    service.add_task(None, None).unwrap();
    let tasks = service.list_tasks(None).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title, "New task");
}

#[test]
fn test_add_task_with_description() {
    let service = test_service();
    service
        .add_task(Some("Task".to_string()), Some("Description".to_string()))
        .unwrap();
    let tasks = service.list_tasks(None).unwrap();
    assert_eq!(tasks[0].description, Some("Description".to_string()));
}

#[test]
fn test_list_tasks_unfiltered() {
    let tasks = vec![
        make_task("1", "Task 1", DefaultStatus::Todo),
        make_task("2", "Task 2", DefaultStatus::Doing),
        make_task("3", "Task 3", DefaultStatus::Done),
    ];
    let service = test_service_with_tasks(tasks);
    let result = service.list_tasks(None).unwrap();
    assert_eq!(result.len(), 3);
}

#[test]
fn test_list_tasks_filtered() {
    let tasks = vec![
        make_task("1", "Task 1", DefaultStatus::Todo),
        make_task("2", "Task 2", DefaultStatus::Doing),
        make_task("3", "Task 3", DefaultStatus::Done),
        make_task("4", "Task 4", DefaultStatus::Todo),
    ];
    let service = test_service_with_tasks(tasks);
    let result = service.list_tasks(Some(DefaultStatus::Todo)).unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|t| t.status == DefaultStatus::Todo));
}

#[test]
fn test_update_status_not_found() {
    let service = test_service();
    let result = service.update_status("missing", DefaultStatus::Done);
    assert!(result.is_err());
}

#[test]
fn test_edit_task_not_found() {
    let service = test_service();
    let result = service.edit_task("nope", Some("Title".to_string()), None);
    assert!(result.is_err());
}

#[test]
fn test_delete_task() {
    let tasks = vec![
        make_task("1", "Keep", DefaultStatus::Todo),
        make_task("2", "Delete", DefaultStatus::Todo),
    ];
    let service = test_service_with_tasks(tasks);
    service.delete_task("2").unwrap();
    let result = service.list_tasks(None).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "1");
}

#[test]
fn test_delete_task_not_found() {
    let service = test_service();
    let result = service.delete_task("ghost");
    assert!(result.is_err());
}
