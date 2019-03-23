use crate::data_gateway_adapter::{RawThread, DataGatewayAdapter, MessageCreationParams, ThreadCreationParams, RawBoard, BoardCreationParams};
use std::ops::Range;
use crate::test_adapter::TestAdapter;

#[derive(Debug)]
pub struct JsonAdapter {
    auto_sweeping: bool,
    adapter: TestAdapter,
}

impl JsonAdapter {
    pub fn new(path: &str, auto_sweeping: bool) -> JsonAdapter {
        JsonAdapter {
            adapter: TestAdapter::new(path, auto_sweeping),
            auto_sweeping,
        }
    }
}

impl DataGatewayAdapter for JsonAdapter {
    fn show_board(&self, board_id: &str) -> Result<RawBoard, String> {
        Ok(self.adapter.show_board(board_id)?)
    }

    fn show_thread(&self, board_id: &str, thread_id: &str, range: Range<usize>) -> Result<RawThread, String> {
        Ok(self.adapter.show_thread(board_id, thread_id, range)?)
    }

    fn create_board(&mut self, params: BoardCreationParams<'_>) -> Result<String, String> {
        Ok(self.adapter.create_board(params)?)
    }

    fn create_thread(&mut self, params: ThreadCreationParams<'_>) -> Result<String, String> {
        Ok(self.adapter.create_thread(params)?)
    }

    fn create_message(&mut self, params: MessageCreationParams<'_>) -> Result<String, String> {
        Ok(self.adapter.create_message(params)?)
    }

    fn lock_thread(&mut self, board_id: &str, thread_id: &str) -> Result<(), String> {
        Ok(self.adapter.lock_thread(board_id, thread_id)?)
    }
}
