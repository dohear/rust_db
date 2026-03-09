use std::io::{self};

// the InputBuffer is used to store tokenized user Input
// as such, stores only one input at a time
// clearing itself at the start of a new read
pub struct InputBuffer {
    pub buffer: Vec<String>,
}

impl InputBuffer {
    pub fn new() -> Self {
        InputBuffer {
            buffer: vec![String::new()],
        }
    }
    pub fn read_input(&mut self) {
        self.buffer.clear();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        line = line.trim().to_string();
        self.buffer = line
            .split_whitespace()
            .map(|s| s.to_string()) // Convert &str to String
            .collect();
    }
}
