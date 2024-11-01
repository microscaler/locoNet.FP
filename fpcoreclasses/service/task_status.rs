use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "enqueued")]
    Enqueued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "finished")]
    Finished,
    #[serde(rename = "timeout")]
    Timeout,
}
