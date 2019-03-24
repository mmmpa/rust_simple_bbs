use std::ops::Range;
use crate::common_error::BoxedError;

pub trait DataGatewayAdapter: Send + Sync {
    fn show_board(&self, board_id: &str) -> Result<RawBoard, BoxedError>;
    fn show_thread(&self, board_id: &str, thread_id: &str, range: Range<usize>) -> Result<RawThread, BoxedError>;

    fn create_board(&self, params: BoardCreationParams<'_>) -> Result<String, BoxedError>;
    fn create_thread(&self, params: ThreadCreationParams<'_>) -> Result<String, BoxedError>;
    fn create_message(&self, params: MessageCreationParams<'_>) -> Result<String, BoxedError>;

    fn close_thread(&self, board_id: &str, thread_id: &str) -> Result<(), BoxedError>;
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
pub struct RawThreadSummary {
    pub title: String,
    pub id: String,
}

#[derive(Debug)]
pub struct RawBoard {
    pub title: String,
    pub summaries: Vec<RawThreadSummary>,
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
