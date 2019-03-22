use crate::message_arrangement::MessageArrangement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadMessage {
    pub index: usize,
    pub raw: String,
    pub html: String,
    pub single_anchors: Vec<usize>,
    pub range_anchors: Vec<(usize, usize)>,
}

impl ThreadMessage {
    pub fn new(raw: &str) -> ThreadMessage {
        ThreadMessage {
            raw: raw.to_string(),
            ..ThreadMessage::default()
        }
    }
    pub fn retrieve(index: usize, raw: String, html: String, single_anchors: Vec<usize>, range_anchors: Vec<(usize, usize)>) -> ThreadMessage {
        ThreadMessage { index, raw, html, single_anchors, range_anchors }
    }

    fn default() -> ThreadMessage {
        ThreadMessage {
            index: 0,
            raw: String::new(),
            html: String::new(),
            single_anchors: vec![],
            range_anchors: vec![],
        }
    }

    fn from_raw(raw: String) -> Result<ThreadMessage, String> {
        let (html, single_anchors, range_anchors) = MessageArrangement::execute(&raw)?;

        Ok(ThreadMessage { raw, html, single_anchors, range_anchors, ..ThreadMessage::default() })
    }
}

#[test]
fn test_render() {
    let m = ThreadMessage::from_raw("".to_string()).unwrap();
    assert_eq!(m.html, "");

    let m = ThreadMessage::from_raw(r###"
        >1
        >01
        >10
    "###.to_string()).unwrap();
    assert_eq!(m.html, r###"
        <a href="#1">&gt;1</a>
        <a href="#1">&gt;1</a>
        <a href="#10">&gt;10</a>
    "###);
}

