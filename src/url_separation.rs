use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Matcher<T: Clone + Send + Sync> {
    matching: Option<T>,
    matching_params: Option<Vec<String>>,
    children: Option<HashMap<String, Matcher<T>>>,
}

const PLACEHOLDER: &str = ":";

impl<T: Clone + Send + Sync> Matcher<T> {
    pub fn new(matching: Option<T>) -> Self {
        Matcher {
            matching,
            matching_params: None,
            children: None,
        }
    }

    pub fn add(&mut self, base: &str, matching: T) {
        let (levels, keys) = separate(base);
        self.add_h(&levels[..], keys, matching);
    }

    fn add_h(&mut self, levels: &[String], keys: Vec<String>, matching: T) {
        if levels.len() == 0 {
            self.matching = Some(matching);
            self.matching_params = Some(keys);
            return;
        }

        let children = match &mut self.children {
            Some(chldren) => chldren,
            None => {
                self.children = Some(HashMap::new());
                self.children.as_mut().unwrap()
            }
        };

        let next = &levels[0];

        let child = match children.get_mut(next) {
            Some(child) => child,
            None => {
                children.insert(next.to_string(), Matcher::new(None));
                children.get_mut(next).unwrap()
            }
        };

        child.add_h(&levels[1..], keys, matching);
    }

    pub fn pick(&self, paths: &[&str]) -> Option<(T, HashMap<String, String>)> {
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
            None => match children.get(PLACEHOLDER) {
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
                if !in_key { in_key = true }
                head += 1;
            },
            '/' => {
                let key_name = base[head..i].to_string();
                head = i + 1;
                if in_key {
                    keys.push(key_name);
                    lebels.push(PLACEHOLDER.to_string());
                    in_key = false;
                } else {
                    lebels.push(key_name);
                }
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

#[cfg(test)]
mod tests {
    use crate::url_separation::Matcher;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn test_string() {
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

    #[test]
    fn test_callback() {
        let mut m = Matcher::new(None);
        m.add("aaa/bbb", Arc::new(|| "test".to_string()));

        let (matching, _) = m.pick(&vec!["aaa", "bbb"]).unwrap();
        assert_eq!(matching(), "test".to_string());
    }
}
