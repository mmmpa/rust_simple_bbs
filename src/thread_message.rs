use regex::{Regex, Error};
use std::str::FromStr;

pub struct ThreadMessage {
    raw: String,
    pub html: String,
    to_them: Vec<u8>,
    to_me: Vec<u8>,
}

impl ThreadMessage {
    fn new(raw: String) -> ThreadMessage {
        ThreadMessage {
            raw,
            ..ThreadMessage::default()
        }
    }

    fn default() -> ThreadMessage {
        ThreadMessage {
            raw: String::new(),
            html: String::new(),
            to_them: vec![],
            to_me: vec![],
        }
    }

    fn render(&mut self) -> Result<(), Error> {
        let reg = Regex::new(r">(?P<number>[1-9]+[0-9]*)")?;
        let replaced = reg.replace_all(&self.raw, r###"<a href="#$number">&gt;$number</a>"###);

        self.html = replaced.to_string();

        Ok(())
    }
}

fn escape(ss: &str) -> String {
    let mut result = String::with_capacity(ss.len());
    let mut last = 0;

    for (i, s) in ss.bytes().enumerate() {
        match s as char {
            '>' | '<' => {
                result.push_str(&ss[last..i]);

                let escaped = match s as char {
                    '>' => "&gt;",
                    '<' => "&lt;",
                    _ => ""
                };

                result.push_str(escaped);
                last = i + 1;
            },
            _ => (),
        }
    }

    result.push_str(&ss[last..ss.len()]);
    result
}

#[derive(Debug)]
enum EscapeState {
    PickingStarter,
    PickingStarterHavingNum,
    PickingFinisher,
    PickingFinisherHavingNum,
    Picking,
}

struct Anchor<'a> {
    raw_body: &'a str,
    state: EscapeState,
    start: Option<usize>,
    end: Option<usize>,
    last: usize,
    result: String,
    stored_string: String,
    num_string: String,
    single_anchors: Vec<usize>,
    range_anchors: Vec<(usize,usize)>,
}

impl<'a> Anchor<'a> {
    fn new(raw_body: &str) -> Anchor {
        Anchor {
            raw_body,
            state: EscapeState::Picking,
            start: None,
            end: None,
            last: 0,
            result: String::with_capacity(8),
            stored_string: String::with_capacity(8),
            num_string: String::with_capacity(4),
            single_anchors: vec![],
            range_anchors: vec![],
        }
    }

    pub fn start_picking(&mut self, now: usize) {
        match self.state {
            EscapeState::PickingStarter => {
                self.use_stored_string(now);
                self.fill(now);
            },
            _ => {
                self.step(now);
                self.fill(now)
            },
        }
        self.stored_string.push_str("&gt;");
        self.last = now + 1;
        self.state = EscapeState::PickingStarter
    }

    pub fn escape_lt(&mut self, now: usize) {
        self.step(now);
        self.fill(now);
        self.result.push_str("&lt;");
        self.last = now + 1;
    }

    fn use_stored_string(&mut self, now: usize) {
        self.result.push_str(&self.stored_string);
        self.stored_string.clear();
    }

    fn fill(&mut self, now: usize) {
        self.result.push_str(&self.raw_body[self.last as usize..now]);
    }

    fn step(&mut self, now: usize) -> Result<(), std::num::ParseIntError> {
        match self.state {
            EscapeState::PickingStarter => {
                self.use_stored_string(now);
            },
            EscapeState::PickingStarterHavingNum => {
                self.start = Some(usize::from_str(&self.num_string)?);
                self.result.push_str(
                    &format!(
                        r###"<a href="#{0:?}">&gt;{0:?}</a>"###,
                        self.start.unwrap(),
                    )
                );
                self.store_anchors_and_reset(now);
            },
            EscapeState::PickingFinisher => {
                self.result.push_str(
                    &format!(
                        r###"<a href="#{0:?}">&gt;{0:?}</a>-"###,
                        self.start.unwrap(),
                    )
                );
                self.store_anchors_and_reset(now);
            },
            EscapeState::PickingFinisherHavingNum => {
                self.end = Some(usize::from_str(&self.num_string)?);
                self.result.push_str(
                    &format!(
                        r###"<a href="#{0:?}-{1:?}">&gt;{0:?}-{1:?}</a>"###,
                        self.start.unwrap(),
                        self.end.unwrap(),
                    ),
                );
                self.store_anchors_and_reset(now);
            },
            _ => (),
        }

        self.state = EscapeState::Picking;

        Ok(())
    }

    fn store_anchors_and_reset(&mut self, now:usize) {
        match (self.start, self.end) {
            (Some(start), Some(end)) => self.range_anchors.push((start, end)),
            (Some(start), None) => self.single_anchors.push(start),
            _ => (),
        }

        self.start = None;
        self.end = None;
        self.num_string.clear();
        self.stored_string.clear();
        self.last = now;
    }

    fn start_range(&mut self, now: usize) {
        match self.state {
            EscapeState::PickingStarterHavingNum => {
                self.start = Some(usize::from_str(&self.num_string).unwrap());
                self.num_string.clear();
                self.state = EscapeState::PickingFinisher;
            },
            _ => {
                self.step(now);
            },
        };
    }

    fn accumulate(&mut self, now: usize, c: char) {
        match self.state {
            EscapeState::PickingStarter => {
                self.num_string.push(c);
                self.state = EscapeState::PickingStarterHavingNum;
            },
            EscapeState::PickingFinisher => {
                self.num_string.push(c);
                self.state = EscapeState::PickingFinisherHavingNum;
            },
            EscapeState::PickingStarterHavingNum | EscapeState::PickingFinisherHavingNum => {
                self.num_string.push(c);
            },
            _ => (),
        }
    }
}

fn escape_and_anchor(raw_body: &str) -> String {
    let mut anchor = Anchor::new(raw_body);
    let mut last = 0;

    for (i, c) in raw_body.bytes().enumerate() {
        match c as char {
            '>' | '-' | '0'...'9' => {
                let c = c as char;
                match c {
                    '>' => anchor.start_picking(i),
                    '-' => anchor.start_range(i),
                    '0'...'9' => anchor.accumulate(i, c),
                    _ => ()
                };
            },
            '<' => anchor.escape_lt(i),
            _ => {
                if let Err(err) = anchor.step(i) {
                    println!("{:?}", err)
                };
            },
        }
    }

    anchor.step(raw_body.len());
    anchor.fill(raw_body.len());

    anchor.result
}

#[test]
fn test_escape() {
    let s = "aaa<bbb>ccc";
    let result = escape(s);
    assert_eq!(result, "aaa&lt;bbb&gt;ccc")
}

#[test]
fn test_escape_and_anchor() {
    let s = r###"
    aaa<bbb>11ccc
    "###;
    let result = escape_and_anchor(s);
    assert_eq!(result, r###"
    aaa&lt;bbb<a href="#11">&gt;11</a>ccc
    "###);

    let s = r###"
    aaa<bbb>11ccc
    >>>>22
    >100-200
    >100-
    >10s0-
    >100>100
    "###;
    let result = escape_and_anchor(s);
    assert_eq!(result, r###"
    aaa&lt;bbb<a href="#11">&gt;11</a>ccc
    &gt;&gt;&gt;<a href="#22">&gt;22</a>
    <a href="#100-200">&gt;100-200</a>
    <a href="#100">&gt;100</a>-
    <a href="#10">&gt;10</a>s0-
    <a href="#100">&gt;100</a><a href="#100">&gt;100</a>
    "###);
}

#[test]
fn range_test() {
    match '1' {
        '0'...'9' => println!("ok"),
        _ => println!("ng")
    }
}

#[test]
fn test_render() {
    let mut m = ThreadMessage::new("".to_string());
    m.render();
    assert_eq!(m.html, "");

    let mut m = ThreadMessage::new(r###"
    >1
    >01
    >10
    "###.to_string());
    m.render();
    assert_eq!(m.html, r###"
    <a href="#1">&gt;1</a>
    >01
    <a href="#10">&gt;10</a>
    "###);
}
