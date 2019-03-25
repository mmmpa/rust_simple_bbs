use crate::message_arrangement::MessageArrangement;
use serde::{Deserialize, Serialize};
use crate::common_error::{CommonError, BoxedError};

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadMessage {
    pub index: usize,
    pub raw: String,
    pub html: String,
    pub single_anchors: Vec<usize>,
    pub range_anchors: Vec<(usize, usize)>,
}

impl ThreadMessage {
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

    pub fn from_raw(raw: &str) -> Result<ThreadMessage, BoxedError> {
        if raw.len() == 0 {
            return Err(CommonError::new_boxed("message", "required"));
        }

        let (html, single_anchors, range_anchors) = MessageArrangement::execute(raw, true)?;

        Ok(ThreadMessage { raw: raw.to_string(), html, single_anchors, range_anchors, ..ThreadMessage::default() })
    }
}

#[test]
fn test_render() {
    let m = ThreadMessage::from_raw(" ").unwrap();
    assert_eq!(m.html, "");

    let m = ThreadMessage::from_raw(r###">1"###).unwrap();
    assert_eq!(m.html, r###"<p><a href="#1">&gt;1</a></p>
"###);
}

