extern crate pulldown_cmark;

use std::str::FromStr;
use crate::common_error::{OrError, BoxedError};
use self::pulldown_cmark::Parser;
use self::pulldown_cmark::html::push_html;

pub struct MessageArrangement<'a> {
    raw_body: &'a str,
    state: EscapeState,
    start: usize,
    end: usize,
    last: usize,
    result: String,
    stored_string: String,
    num_string: String,
    single_anchors: Vec<usize>,
    range_anchors: Vec<(usize, usize)>,
}

#[derive(Debug)]
enum EscapeState {
    PickingStarter,
    PickingStarterHavingNum,
    PickingFinisher,
    PickingFinisherHavingNum,
    Picking,
}

impl<'a> MessageArrangement<'a> {
    fn new(raw_body: &str) -> MessageArrangement<'_> {
        MessageArrangement {
            raw_body,
            state: EscapeState::Picking,
            start: 0,
            end: 0,
            last: 0,
            result: String::with_capacity(raw_body.len()),
            stored_string: String::with_capacity(8),
            num_string: String::with_capacity(4),
            single_anchors: vec![],
            range_anchors: vec![],
        }
    }

    pub fn execute(raw_body: &str, with_md: bool) -> Result<(String, Vec<usize>, Vec<(usize, usize)>), BoxedError> {
        let mut anchor = MessageArrangement::new(raw_body);

        for (i, c) in raw_body.bytes().enumerate() {
            match c as char {
                '>' | '-' | '0'..='9' => {
                    let c = c as char;
                    match c {
                        '>' => anchor.start_picking(i)?,
                        '-' => anchor.start_range(i)?,
                        '0'..='9' => anchor.accumulate(i, c)?,
                        _ => ()
                    };
                },
                '<' => anchor.escape_lt(i)?,
                _ => anchor.step(i)?
            }
        }

        anchor.step(raw_body.len())?;
        anchor.fill(raw_body.len());

        match with_md {
            false => Ok((anchor.result, anchor.single_anchors, anchor.range_anchors)),
            true => {
                let parser = Parser::new(&anchor.result);
                let mut html_buf = String::with_capacity(anchor.result.len() * 2);
                push_html(&mut html_buf, parser);

                Ok((html_buf, anchor.single_anchors, anchor.range_anchors))
            },
        }
    }

    fn start_picking(&mut self, now: usize) -> Result<(), BoxedError> {
        match self.state {
            EscapeState::PickingStarter => {
                self.result.push_str("&gt;");
            },
            _ => {
                self.step(now)?;
                self.fill(now);
            },
        }
        // &gt; is inserted here after a sequence started now.
        self.last = now + 1;
        self.state = EscapeState::PickingStarter;

        Ok(())
    }

    fn escape_lt(&mut self, now: usize) -> Result<(), BoxedError> {
        self.step(now)?;
        self.fill(now);
        self.result.push_str("&lt;");
        self.last = now + 1;

        Ok(())
    }

    fn fill(&mut self, now: usize) {
        self.result.push_str(&self.raw_body[self.last as usize..now]);
    }

    fn step(&mut self, now: usize) -> Result<(), BoxedError> {
        match self.state {
            EscapeState::PickingStarter => {
                self.result.push_str("&gt;");
            },
            EscapeState::PickingStarterHavingNum => {
                self.start = usize::from_str(&self.num_string).or_err("system", "invalid num")?;
                self.result.push_str(
                    &format!(
                        r###"<a href="#{0:?}">&gt;{0:?}</a>"###,
                        self.start,
                    )
                );
                self.single_anchors.push(self.start);
                self.reset(now);
            },
            EscapeState::PickingFinisher => {
                self.result.push_str(
                    &format!(
                        r###"<a href="#{0:?}">&gt;{0:?}</a>-"###,
                        self.start,
                    )
                );
                self.single_anchors.push(self.start);
                self.reset(now);
            },
            EscapeState::PickingFinisherHavingNum => {
                self.end = usize::from_str(&self.num_string).or_err("system", "invalid num")?;
                self.result.push_str(
                    &format!(
                        r###"<a href="#{0:?}-{1:?}">&gt;{0:?}-{1:?}</a>"###,
                        self.start,
                        self.end,
                    ),
                );
                self.range_anchors.push((self.start, self.end));
                self.reset(now);
            },
            EscapeState::Picking => return Ok(()),
        }

        self.state = EscapeState::Picking;

        Ok(())
    }

    fn reset(&mut self, now: usize) {
        self.num_string.clear();
        self.stored_string.clear();
        self.last = now;
    }

    fn start_range(&mut self, now: usize) -> Result<(), BoxedError> {
        match self.state {
            EscapeState::PickingStarterHavingNum => {
                self.start = usize::from_str(&self.num_string).or_err("system", "invalid num")?;
                self.num_string.clear();
                self.state = EscapeState::PickingFinisher;
            },
            _ => {
                self.step(now)?;
            },
        };

        Ok(())
    }

    fn accumulate(&mut self, _now: usize, c: char) -> Result<(), BoxedError> {
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

        Ok(())
    }
}

#[test]
fn test_escape_and_anchor() {
    let s = r###"
        aaa<bbb>11ccc
    "###;
    let (result, _, _) = MessageArrangement::execute(s, false).unwrap();
    assert_eq!(result, r###"
        aaa&lt;bbb<a href="#11">&gt;11</a>ccc
    "###);

    let s = r###"
        >>>>22
        >100-200
        >100-
        >10s0-
        >100>100
    "###;

    let (result, single, range) = MessageArrangement::execute(s, false).unwrap();
    assert_eq!(result, r###"
        &gt;&gt;&gt;<a href="#22">&gt;22</a>
        <a href="#100-200">&gt;100-200</a>
        <a href="#100">&gt;100</a>-
        <a href="#10">&gt;10</a>s0-
        <a href="#100">&gt;100</a><a href="#100">&gt;100</a>
    "###);
    assert_eq!(single, vec![22, 100, 10, 100, 100]);
    assert_eq!(range, vec![(100, 200)]);
}
