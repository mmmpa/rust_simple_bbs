use std::fmt::Debug;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use std::fmt;
use std::error::Error;

pub type BoxedError = Box<CommonError>;

#[derive(Debug)]
pub struct CommonError {
    details: String,
    stacking: HashMap<String, Vec<String>>
}

impl CommonError {
    pub fn push(&mut self, key: &str, value: &str) {
        match self.stacking.get_mut(key) {
            Some(stacking) => stacking.push(value.to_string()),
            None => {
                self.stacking.insert(key.to_string(), vec![value.to_string()]);
            },
        };
        self.details = fotmat!("{:?}", self.stacking);
    }

    pub fn new_boxed(key: &str, value: &str) -> BoxedError {
        let mut boxed_error = CommonError::new("");
        boxed_error.push(key, error);
        Box::new(boxed_error)
    }
}

impl CommonError {
    pub fn new(msg: &str) -> CommonError {
        CommonError {
            details: msg.to_string(),
            stacking: HashMap::new(),
        }
    }
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CommonError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub trait OrError<T, E: Debug> {
    fn or_err(self, key: &str, error: &str) -> Result<T, BoxedError>;
}

impl<T, E: Debug> OrError<T, E> for Result<T, E> {
    fn or_err(self, key: &str, error: &str) -> Result<T, BoxedError> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(CommonError::new_boxed(key, error)),
        }
    }
}
