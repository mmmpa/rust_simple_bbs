use std::ops::Range;

pub trait DataGatewayAdapter: Send + Sync {
    fn show_board(&self, board_id: &str) -> Result<RawBoard, String>;
    fn show_thread(&self, board_id: &str, thread_id: &str, range: Range<usize>) -> Result<RawThread, String>;

    fn create_board(&mut self, params: BoardCreationParams<'_>) -> Result<String, String>;
    fn create_thread(&mut self, params: ThreadCreationParams<'_>) -> Result<String, String>;
    fn create_message(&mut self, params: MessageCreationParams<'_>) -> Result<String, String>;

    fn lock_thread(&mut self, board_id: &str, thread_id: &str) -> Result<(), String>;
}

#[derive(Debug)]
pub struct RawMessage {
    pub index: usize,
    pub raw: String,
    pub html: String,
    pub single_anchors: Vec<usize>,
    pub range_anchors: Vec<(usize, usize)>,
}

#[derive(Debug)]
pub struct RawThread {
    pub locked: bool,
    pub title: String,
    pub messages: Vec<RawMessage>,
}

#[derive(Debug)]
pub struct RawThreadInformation {
    pub title: String,
    pub board_thread_id: String,
}

#[derive(Debug)]
pub struct RawBoard {
    pub title: String,
    pub threads: Vec<RawThreadInformation>,
}

#[derive(Debug)]
pub struct BoardCreationParams<'a> {
    pub title: &'a str,
}

#[derive(Debug)]
pub struct ThreadCreationParams<'a> {
    pub board_id: &'a str,
    pub title: &'a str,
    pub first_message: MessageCreationParams<'a>
}

#[derive(Debug)]
pub struct MessageCreationParams<'a> {
    pub board_id: &'a str,
    pub board_thread_id: &'a str,

    pub raw: &'a str,
    pub html: &'a str,
    pub single_anchors: &'a Vec<usize>,
    pub range_anchors: &'a Vec<(usize, usize)>,
}
