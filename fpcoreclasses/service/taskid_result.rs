#[derive(Debug)]
pub struct TaskIdResult {
    pub task_id: String,
}

impl TaskIdResult {
    pub fn new(task_id: String) -> Self {
        TaskIdResult { task_id }
    }
}
