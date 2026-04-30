use std::fmt;

#[derive(Debug)]
pub enum TaskError {
    TaskNotFound(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::TaskNotFound(id) => {
                write!(f, "Task with ID '{}' not found.", id)
            },
        }
    }
}

impl std::error::Error for TaskError {}
