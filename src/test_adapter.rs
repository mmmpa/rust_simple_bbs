use uuid;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::data_gateway_adapter::{RawThread, RawMessage, MessageCreationParams, ThreadCreationParams, RawBoard, BoardCreationParams, RawThreadSummary};
use std::fs::File;
use std::io::{BufReader, BufRead, Seek};
use std::path::Path;
use std::io::Write;
use std::ops::Range;
use std::fs::OpenOptions;
use std::time::SystemTime;
use std::fs;
use std::mem::{replace};
use crate::common_error::{OrError, BoxedError, CommonError};

#[derive(Debug)]
pub struct TestAdapter {
    logs_root_path: String,
    auto_sweeping: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageRow {
    pub raw: String,
    pub html: String,
    pub single_anchors: String,
    pub range_anchors: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ThreadSchema {
    title: String,
    board_thread_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BoardSchema {
    board_id: String,
    title: String,
    summaries: Vec<ThreadSchema>,
}

impl Drop for TestAdapter {
    fn drop(&mut self) {
        if self.auto_sweeping {
            fs::remove_dir_all(&self.logs_root_path).unwrap();
        }
    }
}

fn swap_string(a: &mut String) -> String {
    replace(a, String::new())
}

impl TestAdapter {
    pub fn new(path: &str, auto_sweeping: bool) -> TestAdapter {
        let logs_root_path = format!("./tmp/{}", path);
        if Path::new(&logs_root_path).exists() {
            //
        } else {
            fs::create_dir(&logs_root_path).expect("test adapter initialize error");
        }

        TestAdapter {
            logs_root_path,
            auto_sweeping,
        }
    }

    fn check_path(path: String, exists: bool) -> Result<String, BoxedError> {
        if Path::new(&path).exists() == exists {
            Ok(path)
        } else {
            let mut boxed_error = CommonError::new("");
            boxed_error.push("system", &format!("{}'s exists is not {:?}", path, exists));

            Err(Box::new(boxed_error))
        }
    }

    fn generate_board_path(&self, board_id: &str) -> String {
        format!("{}/{}", self.logs_root_path, board_id)
    }

    fn generate_board_info_path(&self, board_id: &str) -> String {
        format!("{}/{}/index.json", self.logs_root_path, board_id)
    }

    fn generate_thread_log_path(&self, board_id: &str, thread_id: &str) -> String {
        format!("{}/{}/{}.log", self.logs_root_path, board_id, thread_id)
    }

    fn check_board_path(&self, board_id: &str, exists: bool) -> Result<String, BoxedError> {
        let path = self.generate_board_path(board_id);
        Self::check_path(path, exists)
    }

    fn check_board_info_path(&self, board_id: &str, exists: bool) -> Result<String, BoxedError> {
        let path = self.generate_board_info_path(board_id);
        Self::check_path(path, exists)
    }

    fn check_thread_log_path(&self, board_id: &str, thread_id: &str, exists: bool) -> Result<String, BoxedError> {
        let path = self.generate_thread_log_path(board_id, thread_id);
        Self::check_path(path, exists)
    }

    fn register_thread(&self, board_id: &str, board_thread_id: &str, title: &str) -> Result<(), BoxedError> {
        let mut board = self.read_board_schema(board_id)?;
        board.summaries.push(
            ThreadSchema {
                title: title.to_string(),
                board_thread_id: board_thread_id.to_string(),
            }
        );
        self.write_board_schema(&board)?;

        Ok(())
    }

    fn read_board_schema(&self, board_id: &str) -> Result<BoardSchema, BoxedError> {
        let path = self.generate_board_info_path(board_id);
        let json = fs::read_to_string(path).or_err("system", "broken info file")?;
        let schema: BoardSchema = serde_json::from_str(&json).unwrap();

        Ok(schema)
    }

    fn write_board_schema(&self, schema: &BoardSchema) -> Result<(), BoxedError> {
        let path = self.generate_board_info_path(&schema.board_id);
        let board_json = serde_json::to_string(&schema).or_err("system", "parse error")?;
        fs::write(path, board_json).or_err("system", "board schema write error")?;

        Ok(())
    }

    fn params_to_row(message: &MessageCreationParams<'_>) -> Result<String, BoxedError> {
        let message = MessageRow {
            raw: Self::san(message.raw),
            html: Self::san(message.html),
            single_anchors: message.single_anchors.iter().map(|n| format!("{}", n)).collect::<Vec<String>>().join(","),
            range_anchors: message.range_anchors.iter().map(|(h, e)| format!("{}-{}", h, e)).collect::<Vec<String>>().join(","),
        };
        let json = serde_json::to_string(&message).or_err("system", "parse error")?;

        Ok(format!("{}\n", json))
    }

    fn row_to_raw(index: usize, raw: &str) -> Result<RawMessage, BoxedError> {
        let mut message: MessageRow = serde_json::from_str(raw).or_err("system", "parse error")?;
        let raw = RawMessage {
            index,
            raw: swap_string(&mut message.raw),
            html: swap_string(&mut message.html),
            single_anchors: vec![],
            range_anchors: vec![],
        };

        Ok(raw)
    }

    fn san(string: &str) -> String {
        string.to_string()
    }

    fn generate_id() -> String {
        let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        format!("{:?}_{}", t, Uuid::new_v4().to_string())
    }

    pub fn show_board(&self, board_id: &str) -> Result<RawBoard, BoxedError> {
        let mut board = self.read_board_schema(board_id)?;

        Ok(
            RawBoard {
                title: board.title,
                summaries: board.summaries.iter_mut().map(|ThreadSchema { title, board_thread_id }|
                  RawThreadSummary { title: swap_string(title), id: swap_string(board_thread_id) }
                ).collect()
            }
        )
    }

    pub fn show_thread(&self, board_id: &str, thread_id: &str, range: Range<usize>) -> Result<RawThread, BoxedError> {
        let path = self.check_thread_log_path(board_id, thread_id, true)?;
        let thread = File::open(path)
          .or_err("system", "unknown error")?;
        let mut lines = BufReader::new(thread).lines();

        let locked = lines.next()
          .expect("log format error (no lines)")
          .expect("log format error (no locked)") != "";

        let title = lines.next()
          .expect("log format error (no lines)")
          .or_err("system", "log format error (no title)")?;

        let messages = lines
          .enumerate()
          .skip(range.start)
          .take(range.end - range.start)
          .map(|(i, line)| (i, line.unwrap()))
          .map(|(i, line)| Self::row_to_raw(i, &line))
          .filter(|row| row.is_ok())
          .map(|row| row.unwrap())
          .collect();

        Ok(RawThread { locked, title, messages })
    }

    pub fn create_board(&mut self, params: BoardCreationParams<'_>) -> Result<String, BoxedError> {
        let board_id = Self::generate_id();
        let schema = BoardSchema {
            board_id: board_id.clone(),
            title: params.title.to_string(),
            summaries: vec![],
        };

        let path = self.check_board_path(&board_id, false)?;
        fs::create_dir(&path).or_err("system", "board create error")?;

        self.check_board_info_path(&board_id, false)?;
        self.write_board_schema(&schema)?;

        Ok(board_id)
    }

    pub fn create_thread(&mut self, params: ThreadCreationParams<'_>) -> Result<String, BoxedError> {
        let new_thread_id = Self::generate_id();

        let path = self.check_thread_log_path(params.board_id, &new_thread_id, false)?;
        let mut thread = OpenOptions::new().create(true).append(true).open(path)
          .or_err("system", "thread open error")?;

        let title = Self::san(params.title);
        let first_row = Self::params_to_row(&params.first_message)?;

        write!(thread, "{}", format!("\n{}\n{}", title, first_row)).unwrap();
        self.register_thread(params.board_id, &new_thread_id, params.title).or_err("system", "registration error")?;

        Ok(new_thread_id)
    }

    pub fn create_message(&mut self, params: MessageCreationParams<'_>) -> Result<String, BoxedError> {
        let path = self.check_thread_log_path(params.board_id, params.board_thread_id, true)?;
        let mut thread = OpenOptions::new().read(true).append(true).open(path)
          .or_err("system", "thread open error")?;

        let mut lines = BufReader::new(&thread).lines();

        let locked = lines.next()
          .expect("log format error (no lines)")
          .expect("log format error (no locked)") != "";

        if locked {
            return Err(CommonError::new_boxed("system", "locked"));
        }

        let row = Self::params_to_row(&params)?;

        write!(thread, "{}", row)
          .or_err("system", "message write error")?;

        Ok(params.board_thread_id.to_string())
    }

    pub fn lock_thread(&mut self, board_id: &str, thread_id: &str) -> Result<(), BoxedError> {
        let path = self.check_thread_log_path(board_id, thread_id, true)?;
        let mut thread = OpenOptions::new().write(true).open(path)
          .or_err("system", "thread open error")?;

        thread.seek(std::io::SeekFrom::Start(0)).or_err("system", "lock file failure")?;
        write!(thread, "{}", "locked\n")
          .or_err("system", "message write error")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_adapter::TestAdapter;
    use crate::data_gateway_adapter::{MessageCreationParams, ThreadCreationParams, BoardCreationParams, RawThreadSummary, RawThread};

    #[test]
    fn test_create_board() {
        let mut adapter = TestAdapter::new("test_create_board", true);
        let board_id = &adapter.create_board(BoardCreationParams { title: "test_create_board" }).unwrap();
        let raw_board = adapter.show_board(board_id).unwrap();

        assert_eq!(raw_board.title, "test_create_board");
    }

    #[test]
    fn test_create_thread() {
        let mut adapter = TestAdapter::new("test_create_thread", true);
        let board_id = &adapter.create_board(BoardCreationParams { title: "test_create_thread" }).unwrap();
        let board_thread_id_1 = adapter.create_thread(
            ThreadCreationParams {
                board_id,
                title: "test_create_thread_1",
                first_message: MessageCreationParams {
                    board_id: "",
                    board_thread_id: "",
                    raw: "raw1",
                    html: "html",
                    single_anchors: &vec![1],
                    range_anchors: &vec![(2, 3)],
                }
            }
        ).unwrap();
        let board_thread_id_2 = adapter.create_thread(
            ThreadCreationParams {
                board_id,
                title: "test_create_thread_2",
                first_message: MessageCreationParams {
                    board_id: "",
                    board_thread_id: "",
                    raw: "raw2",
                    html: "html",
                    single_anchors: &vec![1],
                    range_anchors: &vec![(2, 3)],
                }
            }
        ).unwrap();

        let raw_board = adapter.show_board(board_id).unwrap();
        assert_eq!(raw_board.summaries.len(), 2);

        let RawThreadSummary { title, id } = &raw_board.summaries[0];
        assert_eq!(*id, board_thread_id_1);
        assert_eq!(title, "test_create_thread_1");

        let RawThreadSummary { title, id } = &raw_board.summaries[1];
        assert_eq!(*id, board_thread_id_2);
        assert_eq!(title, "test_create_thread_2");

        let RawThread { locked, title, messages } = adapter.show_thread(&board_id, &board_thread_id_1, 0..100).unwrap();
        assert!(!locked);
        assert_eq!(title, "test_create_thread_1");
        assert_eq!(messages[0].raw, "raw1");

        let RawThread { locked, title, messages } = adapter.show_thread(&board_id, &board_thread_id_2, 0..100).unwrap();
        assert!(!locked);
        assert_eq!(title, "test_create_thread_2");
        assert_eq!(messages[0].raw, "raw2");
    }

    #[test]
    fn test_create_messages() {
        let mut adapter = TestAdapter::new("test_create_messages", true);
        let board_id = &adapter.create_board(BoardCreationParams { title: "test_create_messages" }).unwrap();
        let board_thread_id = adapter.create_thread(
            ThreadCreationParams {
                board_id,
                title: "test_create_messages",
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

        adapter.create_message(
            MessageCreationParams {
                board_id,
                board_thread_id: &board_thread_id,
                raw: "message_1",
                html: "message_1",
                single_anchors: &vec![1],
                range_anchors: &vec![(2, 3)],
            }
        ).unwrap();
        adapter.create_message(
            MessageCreationParams {
                board_id,
                board_thread_id: &board_thread_id,
                raw: "message\n_2",
                html: "message\n_2",
                single_anchors: &vec![1, 2, 3],
                range_anchors: &vec![(2, 3), (3, 4)],
            }
        ).unwrap();

        let RawThread { messages, .. } = adapter.show_thread(&board_id, &board_thread_id, 0..100).unwrap();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[1].raw, "message_1");
        assert_eq!(messages[2].raw, "message\n_2");

        let RawThread { messages, .. } = adapter.show_thread(&board_id, &board_thread_id, 1..2).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].raw, "message_1");

        let RawThread { messages, .. } = adapter.show_thread(&board_id, &board_thread_id, 2..100).unwrap();
        assert_eq!(messages[0].raw, "message\n_2");

        adapter.lock_thread(board_id, &board_thread_id).unwrap();

        assert!(adapter.create_message(
            MessageCreationParams {
                board_id,
                board_thread_id: &board_thread_id,
                raw: "message_3",
                html: "message_3",
                single_anchors: &vec![1, 2, 3],
                range_anchors: &vec![(2, 3), (3, 4)],
            }
        ).is_err());
    }
}
