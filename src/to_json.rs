use crate::board::Board;
use crate::board_thread::BoardThread;

pub trait ToJson{
    fn to_json(&self) -> String;
}

impl ToJson for Board {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl ToJson for BoardThread {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
