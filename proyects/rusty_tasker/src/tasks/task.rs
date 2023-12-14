pub struct Task {
    pub id: u64,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u64, description: &str) -> Self {
        Task {
            id,
            description: description.to_string(),
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

