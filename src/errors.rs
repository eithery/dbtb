use std::error::Error;
use std::fmt::{Display, Formatter};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ErrorKind {
    IOError,
    YamlDeserializationError
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct DbtbError {
    message: String,
    kind: ErrorKind,
    file_path: Option<String>,
    inner_error: Option<Box<dyn Error + Send + Sync>>
}


impl Display for DbtbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}


impl Error for DbtbError {
}


impl From<std::io::Error> for DbtbError {
    fn from(err: std::io::Error) -> Self {
        DbtbError::from_error(err, ErrorKind::IOError, "IO error")
    }
}


impl From<serde_yaml::Error> for DbtbError {
    fn from(err: serde_yaml::Error) -> Self {
        DbtbError::from_error(err, ErrorKind::YamlDeserializationError, "YAML deserialization")
    }
}


#[allow(dead_code)]
impl DbtbError {
    pub(crate) fn new<S: Into<String>>(message: S, kind: ErrorKind) -> Self {
        Self {
            message: message.into(),
            kind,
            file_path: None,
            inner_error: None
        }
    }


    fn from_error<E>(error: E, kind: ErrorKind, prefix: &str) -> Self
        where E: Error + Send + Sync + 'static
    {
        Self {
            message: format!("{prefix}. {error}"),
            kind,
            file_path: None,
            inner_error: Some(Box::new(error))
        }
    }


    fn kind(&self) -> ErrorKind {
        self.kind
    }
}
