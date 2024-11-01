use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

// Assuming TaskStatus is already defined as an enum
#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Unknown,
    Enqueued,
    Running,
    Finished,
    // Add other variants as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskInfoResult {
    /// The current status of the task
    #[serde(rename = "TaskStatus")]
    pub task_status: TaskStatus,
    /// The result of the task
    pub result: Option<serde_json::Value>, // Using serde_json::Value for flexibility
}

impl TaskInfoResult {
    pub fn new(task_status: TaskStatus, result: Option<serde_json::Value>) -> Self {
        TaskInfoResult {
            task_status,
            result,
        }
    }
}
