use crate::thread_message::ThreadMessage;

#[derive(Debug)]
pub struct BoardThread {
    pub title: String,
    pub messages: Vec<ThreadMessage>,
}

impl BoardThread {
    pub fn retrieve(title: String, messages: Vec<ThreadMessage>) -> BoardThread {
        BoardThread { title, messages }
    }
}
