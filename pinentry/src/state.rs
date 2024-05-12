pub struct State {
    pub description: String,
    pub prompt: String,
}

impl State {
    pub fn new() -> Self {
        Self {
            description: String::new(),
            prompt: String::new(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
