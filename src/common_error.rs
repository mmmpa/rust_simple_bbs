use std::fmt::Debug;

pub trait OrError<T, E: Debug> {
    fn or_err(self, error: &str) -> Result<T, String>;
}

impl<T, E: Debug> OrError<T, E> for Result<T, E> {
    fn or_err(self, error: &str) -> Result<T, String> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(format!("{}: {:?}", error, e)),
        }
    }
}
