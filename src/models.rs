use clap::ValueEnum;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DefaultState {
    Todo,
    Doing,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id:          String,
    pub title:       String,
    pub description: Option<String>,
    pub state:       DefaultState,
    pub created_at:  String,
    pub updated_at:  String,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: nanoid!(8),
            state: DefaultState::Todo,
            created_at: chrono::Local::now().to_rfc2822(),
            updated_at: chrono::Local::now().to_rfc2822(),
            description: Some(String::new()),
            title,
        }
    }

    pub fn update_time(&mut self) {
        self.updated_at = chrono::Local::now().to_rfc2822();
    }
}
