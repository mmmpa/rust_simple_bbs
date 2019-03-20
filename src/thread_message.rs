use crate::message_arrangement::MessageArrangement;

#[derive(Debug)]
pub struct ThreadMessage {
    pub raw: String,
    pub html: String,
    pub single_anchors: Vec<usize>,
    pub range_anchors: Vec<(usize, usize)>,
}

impl ThreadMessage {
    fn new(raw: String) -> ThreadMessage {
        ThreadMessage {
            raw,
            ..ThreadMessage::default()
        }
    }
    pub fn retrieve(index: usize, raw: String, html: String, single_anchors: Vec<usize>, range_anchors: Vec<(usize, usize)>) -> ThreadMessage {
        ThreadMessage { raw, html, single_anchors, range_anchors }
    }

    fn default() -> ThreadMessage {
        ThreadMessage {
            raw: String::new(),
            html: String::new(),
            single_anchors: vec![],
            range_anchors: vec![],
        }
    }

    fn from_raw(raw: String) -> Result<ThreadMessage, String> {
        let (html, single_anchors, range_anchors) = MessageArrangement::execute(&raw)?;

        Ok(ThreadMessage { raw, html, single_anchors, range_anchors })
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

