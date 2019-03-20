use crate::board_thread::BoardThread;
use crate::thread_message::ThreadMessage;
use crate::board::Board;
use crate::data_gateway_adapter::{DataGatewayAdapter, RawThread, RawMessage, ThreadCreationParams, MessageCreationParams};
use std::mem::{swap, replace};
use std::ops::Range;

pub struct DataGateway<'a> {
    pub adapter: &'a DataGatewayAdapter,
}

impl<'a> DataGateway<'a> {
    pub fn new(adapter: &DataGatewayAdapter) -> DataGateway {
        DataGateway { adapter }
    }

    pub fn show_board(&self) -> Board {
        Board {}
    }

    pub fn create_thread(&self, board: ThreadCreationParams) -> Result<(), ()> {
        self.adapter.create_thread(board);
        Ok(())
    }

    pub fn show_thread(&self, board_id: &str, thread_id: &str, range: Range<usize>) -> Result<BoardThread, String> {
        let RawThread { title, mut messages } = self.adapter.show_thread(board_id, thread_id, range)?;

        let messages = messages.iter_mut().map(|RawMessage { index, raw, html, single_anchors, range_anchors }| {
            ThreadMessage::retrieve(
                index.clone(),
                swap_string(raw),
                swap_string(html),
                swap_vec(single_anchors),
                swap_vec(range_anchors),
            )
        }).collect();
        Ok(BoardThread::retrieve(title, messages))
    }

    pub fn create_message(&self, message: MessageCreationParams) -> Result<(), String> {
        self.adapter.create_message(message)?;
        Ok(())
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
    use crate::data_gateway_adapter::{MessageCreationParams, ThreadCreationParams};

    #[test]
    fn test_fetch() {
        let adapter = &TestAdapter::new("test1", true);
        let gate = DataGateway { adapter };

        assert_eq!(gate.show_thread("1", "1", 0..100).unwrap().title, "dummy title");
        assert_eq!(gate.show_thread("1", "1", 0..100).unwrap().messages[0].raw, "a");
        assert_eq!(gate.show_thread("1", "1", 0..100).unwrap().messages[0].range_anchors, vec![(1, 2)])
    }

    #[test]
    fn test_create_thre() {
        let adapter = &TestAdapter::new("test2", true);
        let gate = DataGateway { adapter };
        gate.create_thread(
            ThreadCreationParams {
                board_id: "new_board",
                title: "new thread",
                first_message: MessageCreationParams {
                    board_id: "",
                    board_thread_id: "",
                    raw: "raw",
                    html: "html",
                    single_anchors: &vec![1],
                    range_anchors: &vec![(2, 3)],
                }
            }
        ).unwrap();
    }

    //#[test]
    fn test_post() {
        let adapter = &TestAdapter::new("test3", true);
        let gate = DataGateway { adapter };
        gate.create_message(MessageCreationParams {
            board_id: "board_test",
            board_thread_id: "thread_test",
            raw: "raw",
            html: "html",
            single_anchors: &vec![1],
            range_anchors: &vec![(2, 3)],
        }).unwrap();

        assert_eq!(gate.show_thread("1", "1", 0..100).unwrap().title, "dummy title");
        assert_eq!(gate.show_thread("1", "1", 0..100).unwrap().messages[0].raw, "a");
        assert_eq!(gate.show_thread("1", "1", 0..100).unwrap().messages[0].range_anchors, vec![(1, 2)])
    }
}
