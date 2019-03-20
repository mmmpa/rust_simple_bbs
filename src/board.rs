pub struct Board {
    pub title: String,
    pub threads: Vec<ThreadInformation>,
}

#[derive(Debug)]
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
