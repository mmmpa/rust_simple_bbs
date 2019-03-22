use crate::board_thread::BoardThread;
use crate::thread_message::ThreadMessage;
use crate::board::{Board, ThreadInformation};
use crate::data_gateway_adapter::{DataGatewayAdapter, RawThread, RawMessage, ThreadCreationParams, MessageCreationParams, BoardCreationParams, RawBoard, RawThreadInformation};
use std::mem::{replace};
use std::ops::Range;

pub struct DataGateway {
    pub adapter: Box<DataGatewayAdapter>,
}

impl DataGateway {
    pub fn new(adapter: Box<DataGatewayAdapter>) -> DataGateway {
        DataGateway { adapter }
    }

    pub fn show_board(&self, board_id: &str) -> Result<Board, String> {
        let RawBoard { mut title, mut threads } = self.adapter.show_board(board_id)?;
        let threads = threads.iter_mut().map(|RawThreadInformation { title, board_thread_id }| {
            ThreadInformation {
                title: swap_string(title),
                board_thread_id: swap_string(board_thread_id),
            }
        }).collect();
        let board = Board::retrieve(swap_string(&mut title), threads);
        Ok(board)
    }

    pub fn show_thread(&self, board_id: &str, thread_id: &str, range: Range<usize>) -> Result<BoardThread, String> {
        let RawThread { locked, title, mut messages } = self.adapter.show_thread(board_id, thread_id, range)?;

        let messages = messages.iter_mut().map(|RawMessage { index, raw, html, single_anchors, range_anchors }| {
            ThreadMessage::retrieve(
                index.clone(),
                swap_string(raw),
                swap_string(html),
                swap_vec(single_anchors),
                swap_vec(range_anchors),
            )
        }).collect();
        Ok(BoardThread::retrieve(locked, title, messages))
    }

    pub fn create_board(&mut self, title: &str) -> Result<String, String> {
        if title.len() == 0 {
            return Err("title required".to_string());
        }
        self.adapter.create_board(BoardCreationParams { title })
    }

    pub fn create_thread(&mut self, board_id: &str, title: &str, message: &ThreadMessage) -> Result<String, String> {
        self.adapter.create_thread(
            ThreadCreationParams {
                board_id,
                title,
                first_message: message_to_params("", "", message)
            }
        )
    }

    pub fn create_message(&mut self, board_id: &str, board_thread_id: &str, message: &ThreadMessage) -> Result<(), String> {
        self.adapter.create_message(
            message_to_params(board_id, board_thread_id, message)
        )?;
        Ok(())
    }
}

fn message_to_params<'a>(board_id: &'a str, board_thread_id: &'a str, message: &'a ThreadMessage) -> MessageCreationParams<'a> {
    let ThreadMessage { raw, html, single_anchors, range_anchors, .. } = message;
    MessageCreationParams {
        board_id,
        board_thread_id,
        raw,
        html,
        single_anchors,
        range_anchors,
    }
}

fn swap_string(a: &mut String) -> String {
    replace(a, String::new())
}

fn swap_vec<T>(a: &mut Vec<T>) -> Vec<T> {
    replace(a, Vec::new())
}

#[cfg(test)]
mod tests {
    use crate::test_adapter::TestAdapter;
    use crate::data_gateway::DataGateway;
    use crate::board::ThreadInformation;
    use crate::board_thread::BoardThread;
    use crate::thread_message::ThreadMessage;
    use crate::json_adapter::JsonAdapter;

    #[test]
    fn test_create_board() {
        let adapter = JsonAdapter::new("test_create_board_gate", true);
        let mut gate = DataGateway::new(Box::new(adapter));
        let board_id = &gate.create_board("test_create_board_gate").unwrap();
        let board = &gate.show_board(board_id).unwrap();

        assert_eq!(board.title, "test_create_board_gate");
    }

    #[test]
    fn test_create_thread() {
        let adapter = JsonAdapter::new("test_create_thread_gate", true);
        let mut gate = DataGateway::new(Box::new(adapter));
        let board_id = &gate.create_board("test_create_thread_gate").unwrap();

        let board_thread_id_1 = &gate.create_thread(
            board_id,
            "test_create_thread_1",
            &ThreadMessage::new("raw1"),
        ).unwrap();
        let board_thread_id_2 = &gate.create_thread(
            board_id,
            "test_create_thread_2",
            &ThreadMessage::new("raw2"),
        ).unwrap();

        let board = &gate.show_board(board_id).unwrap();
        assert_eq!(board.threads.len(), 2);

        let ThreadInformation { title, board_thread_id } = &board.threads[0];
        assert_eq!(board_thread_id, board_thread_id_1);
        assert_eq!(title, "test_create_thread_1");

        let ThreadInformation { title, board_thread_id } = &board.threads[1];
        assert_eq!(board_thread_id, board_thread_id_2);
        assert_eq!(title, "test_create_thread_2");

        let BoardThread { locked, title, messages } = &gate.show_thread(&board_id, &board_thread_id_1, 0..100).unwrap();
        assert!(!locked);
        assert_eq!(title, "test_create_thread_1");
        assert_eq!(messages[0].raw, "raw1");

        let BoardThread { locked, title, messages } = &gate.show_thread(&board_id, &board_thread_id_2, 0..100).unwrap();
        assert!(!locked);
        assert_eq!(title, "test_create_thread_2");
        assert_eq!(messages[0].raw, "raw2");
    }

    #[test]
    fn test_create_messages() {
        let adapter = JsonAdapter::new("test_create_messages_gate", true);
        let mut gate = DataGateway::new(Box::new(adapter));
        let board_id = &gate.create_board("test_create_messages_gate").unwrap();
        let board_thread_id = &gate.create_thread(
            board_id,
            "test_create_messages_gate",
            &ThreadMessage::new("message_1"),
        ).unwrap();

        &gate.create_message(
            board_id,
            board_thread_id,
            &ThreadMessage::new("message_1"),
        ).unwrap();
        &gate.create_message(
            board_id,
            board_thread_id,
            &ThreadMessage::new("message_2"),
        ).unwrap();

        let BoardThread { messages, .. } = &gate.show_thread(&board_id, &board_thread_id, 0..100).unwrap();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[1].raw, "message_1");
        assert_eq!(messages[2].raw, "message_2");

        let BoardThread { messages, .. } = &gate.show_thread(&board_id, &board_thread_id, 1..2).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].raw, "message_1");

        let BoardThread { messages, .. } = &gate.show_thread(&board_id, &board_thread_id, 2..100).unwrap();
        assert_eq!(messages[0].raw, "message_2");
    }
}
