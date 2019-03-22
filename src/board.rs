use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub title: String,
    pub threads: Vec<ThreadInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadInformation {
    pub title: String,
    pub board_thread_id: String,
}

impl Board {
    pub fn retrieve(title: String, threads: Vec<ThreadInformation>) -> Board {
        Board {
            title,
            threads,
        }
    }
}
