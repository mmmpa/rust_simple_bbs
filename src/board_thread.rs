use crate::thread_message::ThreadMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardThread {
    pub locked: bool,
    pub title: String,
    pub messages: Vec<ThreadMessage>,
}

impl BoardThread {
    pub fn retrieve(locked: bool, title: String, messages: Vec<ThreadMessage>) -> BoardThread {
        BoardThread { locked, title, messages }
    }
}
