use crate::data_gateway_adapter::{RawThread, DataGatewayAdapter, MessageCreationParams, ThreadCreationParams, RawBoard, BoardCreationParams};
use std::ops::Range;
use crate::test_adapter::TestAdapter;
use std::sync::RwLock;

#[derive(Debug)]
pub struct JsonAdapter {
    auto_sweeping: bool,
    adapter: RwLock<TestAdapter>,
}

impl JsonAdapter {
    pub fn new(path: &str, auto_sweeping: bool) -> JsonAdapter {
        JsonAdapter {
            adapter: RwLock::new(TestAdapter::new(path, auto_sweeping)),
            auto_sweeping,
        }
    }
}

impl DataGatewayAdapter for JsonAdapter {
    fn show_board(&self, board_id: &str) -> Result<RawBoard, String> {
        Ok(self.adapter.read().unwrap().show_board(board_id)?)
    }

    fn show_thread(&self, board_id: &str, thread_id: &str, range: Range<usize>) -> Result<RawThread, String> {
        Ok(self.adapter.read().unwrap().show_thread(board_id, thread_id, range)?)
    }

    fn create_board(&self, params: BoardCreationParams<'_>) -> Result<String, String> {
        Ok(self.adapter.write().unwrap().create_board(params)?)
    }

    fn create_thread(&self, params: ThreadCreationParams<'_>) -> Result<String, String> {
        Ok(self.adapter.write().unwrap().create_thread(params)?)
    }

    fn create_message(&self, params: MessageCreationParams<'_>) -> Result<String, String> {
        Ok(self.adapter.write().unwrap().create_message(params)?)
    }

    fn close_thread(&self, board_id: &str, thread_id: &str) -> Result<(), String> {
        Ok(self.adapter.write().unwrap().lock_thread(board_id, thread_id)?)
    }
}

