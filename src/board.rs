use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub title: String,
    pub summaries: Vec<ThreadSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadSummary {
    pub title: String,
    pub board_thread_id: String,
}

impl Board {
    pub fn retrieve(title: String, summaries: Vec<ThreadSummary>) -> Board {
        Board {
            title,
            summaries,
        }
    }
}
