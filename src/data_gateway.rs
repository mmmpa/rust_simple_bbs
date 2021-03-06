use crate::board_thread::BoardThread;
use crate::thread_message::ThreadMessage;
use crate::board::{Board, ThreadSummary};
use crate::data_gateway_adapter::{DataGatewayAdapter, RawThread, RawMessage, ThreadCreationParams, MessageCreationParams, BoardCreationParams, RawBoard, RawThreadSummary};
use std::mem::{replace};
use std::ops::Range;
use crate::common_error::{BoxedError, CommonError};

pub struct DataGateway {
    pub adapter: Box<dyn DataGatewayAdapter>,
}

impl DataGateway {
    pub fn new(adapter: Box<dyn DataGatewayAdapter>) -> DataGateway {
        DataGateway { adapter }
    }

    pub fn show_board(&self, board_id: &str) -> Result<Board, BoxedError> {
        let RawBoard { mut title, mut summaries } = self.adapter.show_board(board_id)?;
        let threads = summaries.iter_mut().map(|RawThreadSummary { title, id }| {
            ThreadSummary {
                title: swap_string(title),
                id: swap_string(id),
            }
        }).collect();
        let board = Board::retrieve(swap_string(&mut title), threads);
        Ok(board)
    }

    pub fn show_thread(&self, board_id: &str, thread_id: &str, range: Range<usize>) -> Result<BoardThread, BoxedError> {
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

    pub fn create_board(&self, title: &str) -> Result<String, BoxedError> {
        if title.len() == 0 {
            return Err(CommonError::new_boxed("title", "required"));
        }
        self.adapter.create_board(BoardCreationParams { title })
    }

    pub fn create_thread(&self, board_id: &str, title: &str, message: &str) -> Result<String, BoxedError> {
        self.adapter.create_thread(
            ThreadCreationParams {
                board_id,
                title,
                first_message: message_to_params("", "", &ThreadMessage::from_raw(message)?)
            }
        )
    }

    pub fn create_message(&self, board_id: &str, board_thread_id: &str, message: &str) -> Result<(), BoxedError> {
        self.adapter.create_message(
            message_to_params(board_id, board_thread_id, &ThreadMessage::from_raw(message)?)
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
    use crate::data_gateway::DataGateway;
    use crate::board::ThreadSummary;
    use crate::board_thread::BoardThread;
    use crate::json_adapter::JsonAdapter;

    #[test]
    fn test_create_board() {
        let adapter = JsonAdapter::new("test_create_board_gate", true);
        let gate = DataGateway::new(Box::new(adapter));
        let board_id = &gate.create_board("test_create_board_gate").unwrap();
        let board = &gate.show_board(board_id).unwrap();

        assert_eq!(board.title, "test_create_board_gate");
    }

    #[test]
    fn test_create_thread() {
        let adapter = JsonAdapter::new("test_create_thread_gate", true);
        let gate = DataGateway::new(Box::new(adapter));
        let board_id = &gate.create_board("test_create_thread_gate").unwrap();

        let board_thread_id_1 = &gate.create_thread(
            board_id,
            "test_create_thread_1",
            "raw1",
        ).unwrap();
        let board_thread_id_2 = &gate.create_thread(
            board_id,
            "test_create_thread_2",
            "raw2",
        ).unwrap();

        let board = &gate.show_board(board_id).unwrap();
        assert_eq!(board.summaries.len(), 2);

        let ThreadSummary { title, id } = &board.summaries[0];
        assert_eq!(id, board_thread_id_1);
        assert_eq!(title, "test_create_thread_1");

        let ThreadSummary { title, id } = &board.summaries[1];
        assert_eq!(id, board_thread_id_2);
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
        let gate = DataGateway::new(Box::new(adapter));
        let board_id = &gate.create_board("test_create_messages_gate").unwrap();
        let board_thread_id = &gate.create_thread(
            board_id,
            "test_create_messages_gate",
            "message_1",
        ).unwrap();

        &gate.create_message(
            board_id,
            board_thread_id,
            "message_1",
        ).unwrap();
        &gate.create_message(
            board_id,
            board_thread_id,
            "message_2",
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
