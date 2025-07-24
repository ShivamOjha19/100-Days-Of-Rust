use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}