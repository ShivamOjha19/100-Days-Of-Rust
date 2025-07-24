use crate::task::Task;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const FILE: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    let mut file = match File::open(FILE) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string_pretty(tasks).expect("Serialization failed");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE)
        .expect("Cannot open file");
    file.write_all(data.as_bytes()).expect("Write failed");
}
