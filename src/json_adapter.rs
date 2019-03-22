extern crate uuid;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::data_gateway_adapter::{RawThread, DataGatewayAdapter, RawMessage, MessageCreationParams, ThreadCreationParams, RawBoard, BoardCreationParams, RawThreadInformation};
use std::fs::File;
use std::io::{BufReader, BufRead, Seek};
use std::path::Path;
use std::io::Write;
use std::ops::Range;
use std::fs::OpenOptions;
use std::time::SystemTime;
use std::fs;
use std::mem::{replace};
use crate::test_adapter::TestAdapter;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct JsonAdapter {
    auto_sweeping: bool,
    adapter: Arc<RwLock<TestAdapter>>,
}

impl JsonAdapter {
    pub fn new(path: &str, auto_sweeping: bool) -> JsonAdapter {
        JsonAdapter {
            adapter: Arc::new(
                RwLock::new(
                    TestAdapter::new(path, auto_sweeping)
                )
            ),
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

    fn create_board(&mut self, params: BoardCreationParams) -> Result<String, String> {
        Ok(self.adapter.write().unwrap().create_board(params)?)
    }

    fn create_thread(&mut self, params: ThreadCreationParams) -> Result<String, String> {
        Ok(self.adapter.write().unwrap().create_thread(params)?)
    }

    fn create_message(&mut self, params: MessageCreationParams) -> Result<String, String> {
        Ok(self.adapter.write().unwrap().create_message(params)?)
    }

    fn lock_thread(&mut self, board_id: &str, thread_id: &str) -> Result<(), String> {
        Ok(self.adapter.write().unwrap().lock_thread(board_id, thread_id)?)
    }
}
