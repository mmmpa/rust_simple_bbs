use std::ops::Range;

pub trait DataGatewayAdapter {
    fn show_board(&self, board_id: &str) -> Result<RawBoard, String>;
    fn show_thread(&self, board_id: &str, thread_id: &str, range: Range<usize>) -> Result<RawThread, String>;

    fn create_board(&self, params: BoardCreationParams) -> Result<String, String>;
    fn create_thread(&self, params: ThreadCreationParams) -> Result<String, String>;
    fn create_message(&self, params: MessageCreationParams) -> Result<String, String>;
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

pub struct BoardCreationParams<'a> {
    pub title: &'a str,
}

pub struct ThreadCreationParams<'a> {
    pub board_id: &'a str,
    pub title: &'a str,
    pub first_message: MessageCreationParams<'a>
}

pub struct MessageCreationParams<'a> {
    pub board_id: &'a str,
    pub board_thread_id: &'a str,

    pub raw: &'a str,
    pub html: &'a str,
    pub single_anchors: &'a Vec<usize>,
    pub range_anchors: &'a Vec<(usize, usize)>,
}
