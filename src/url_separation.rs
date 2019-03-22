use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
struct Matcher<T: Clone + Debug> {
    matching: Option<T>,
    matching_params: Option<Vec<String>>,
    children: Option<HashMap<String, Matcher<T>>>,
}

const SEPARATOR: &str = ":";

impl<T: Clone + Debug> Matcher<T> {
    fn new(matching: Option<T>) -> Self {
        Matcher {
            matching,
            matching_params: None,
            children: None,
        }
    }

    fn add(&mut self, base: &str, matching: T) {
        let (levels, keys) = separate(base);
        self.add_h(&levels[..], keys, matching);
    }

    fn add_h(&mut self, levels: &[String], keys: Vec<String>, matching: T) {
        match &self.children {
            Some(_) => (),
            None => self.children = Some(HashMap::new())
        };

        if levels.len() == 0 {
            self.matching = Some(matching);
            self.matching_params = Some(keys);
            return;
        }

        let chldren = self.children.as_mut().unwrap();

        let next = &levels[0];
        match chldren.get_mut(next) {
            Some(c) => c.add_h(&levels[1..], keys, matching),
            None => {
                let mut newer = Matcher::new(None);
                newer.add_h(&levels[1..], keys, matching);
                chldren.insert(next.to_string(), newer);
            }
        }
    }

    fn pick(&self, paths: &[&str]) -> Option<(T, HashMap<String, String>)> {
        self.pick_r(paths, &mut Vec::with_capacity(2))
    }

    fn pick_r(&self, paths: &[&str], values: &mut Vec<String>) -> Option<(T, HashMap<String, String>)> {
        if paths.len() == 0 {
            if self.matching.is_none() {
                return None;
            }

            let matching = self.matching.as_ref().unwrap().clone();

            return match &self.matching_params {
                None => Some((matching, HashMap::new())),
                Some(keys) => Some((matching, build(keys, values))),
            };
        }

        if self.children.is_none() {
            return None;
        }

        let children = self.children.as_ref().unwrap().clone();

        let next = match children.get(paths[0]) {
            None => match children.get(SEPARATOR) {
                None => return None,
                Some(next) => {
                    values.push(paths[0].to_string());
                    next
                }
            },
            Some(next) => next
        };

        next.pick_r(&paths[1..], values)
    }
}

fn separate(base: &str) -> (Vec<String>, Vec<String>) {
    let mut lebels = vec![];
    let mut keys = vec![];
    let mut in_key = false;
    let mut head = 0;

    let checker = match base.chars().last() {
        Some(a) if a != '/' => {
            let mut s = base.to_string();
            s.push('/');
            s
        },
        _ => base.to_string()
    };

    for (i, c) in checker.chars().enumerate() {
        match c {
            ':' => {
                if !in_key {
                    in_key = true;
                }
            },
            '/' => {
                if in_key {
                    let key_name = base[head + 1..i].to_string();
                    println!("{}", key_name);
                    keys.push(key_name);
                    lebels.push(SEPARATOR.to_string());
                    in_key = false;
                } else {
                    let key_name = base[head..i].to_string();
                    lebels.push(key_name);
                }
                head = i + 1;
            }
            _ => ()
        }
    }

    (lebels, keys)
}

fn build(keys: &[String], values: &[String]) -> HashMap<String, String> {
    let mut value_map = HashMap::new();
    keys.iter().enumerate().for_each(|(i, k)| { value_map.insert(k.to_string(), values[i].clone()); });
    value_map
}

#[test]
fn test() {
    let mut m = Matcher::new(Some("".to_string()));
    m.add("aaa/:abc/ddd", "matching DDD".to_string());
    m.add("aaa/bbb", "matching BBB".to_string());

    let (matching, map) = m.pick(&vec!["aaa", "bbb"]).unwrap();
    assert_eq!(matching, "matching BBB".to_string());
    assert_eq!(map, HashMap::new());

    let (matching, map) = m.pick(&vec!["aaa", "DDD", "ddd"]).unwrap();
    let mut expected = HashMap::new();
    expected.insert("abc".to_string(), "DDD".to_string());
    assert_eq!(matching, "matching DDD".to_string());
    assert_eq!(map, expected);

    assert!(m.pick(&vec!["b", "DDD", "ddd"]).is_none());
}
