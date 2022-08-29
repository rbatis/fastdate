use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Error {
    E(String),
}

impl From<&str> for Error {
    fn from(arg: &str) -> Self {
        return Error::E(arg.to_string());
    }
}

impl From<std::string::String> for Error {
    fn from(arg: String) -> Self {
        return Error::E(arg);
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            Error::E(ref err) => {
                write!(f, "{}", err)
            }
        };
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        return match self {
            Error::E(data) => data.as_str(),
        };
    }
}

impl Default for Error {
    fn default() -> Self {
        Error::E(String::new())
    }
}
