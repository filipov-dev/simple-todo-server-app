use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc, serde::{ts_seconds, ts_seconds_option}};
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub(crate) tasks: Arc<Mutex<Vec<Task>>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Task {
    id: usize,
    name: String,
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    finished_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    deleted_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(id: usize, name: String) -> Task {
        Task { id, name, created_at: Utc::now(), updated_at: Utc::now(), finished_at: None, deleted_at: None }
    }

    pub fn update_name(&mut self, new_name: String) -> () {
        self.name = new_name;
        self.created_at = Utc::now();
    }

    pub fn toggle_status(&mut self) -> () {
        if self.finished_at.is_none() {
            self.finished_at = Option::from(Utc::now());
        } else {
            self.finished_at = None;
        }
    }

    pub fn delete(&mut self) -> () {
        if self.deleted_at.is_none() {
            self.deleted_at = Option::from(Utc::now());
        } else {
            self.deleted_at = None;
        }
    }
}