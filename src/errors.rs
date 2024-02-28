use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

/// Error returned by the Clock when trying to exit a subroutine that has not been started.
#[derive(Debug)]
pub struct ClockError {
    pub message: String,
}

impl ClockError {
    pub fn new(message: &str) -> ClockError {
        ClockError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ClockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ClockError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
pub struct InvalidClqFileFormat {
    pub message: String,
}

impl InvalidClqFileFormat {
    pub fn new(message: &str) -> InvalidClqFileFormat {
        InvalidClqFileFormat {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for InvalidClqFileFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InvalidClqFileFormat {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<io::Error> for InvalidClqFileFormat {
    fn from(err: io::Error) -> Self {
        InvalidClqFileFormat::new(&err.to_string())
    }
}

impl From<ParseIntError> for InvalidClqFileFormat {
    fn from(err: ParseIntError) -> Self {
        InvalidClqFileFormat::new(&err.to_string())
    }
}


pub enum YamlError {
    /// Error returned when there is an error while creating / searching a file.
    IoError(String, io::Error),
    /// Error returned when an object is not found in the YAML file.
    NotFound(String, String),
    /// Error returned when an error occurs while parsing the YAML file.
    YAMLParsingError(String, serde_yaml::Error),
    /// Error returned when the YAML file is not formatted correctly.
    YAMLFormatError(String, serde_yaml::Error),
}

impl fmt::Display for YamlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YamlError::IoError(msg, _err) => write!(f, "{}", msg),
            YamlError::NotFound(msg, _err) => write!(f, "{}", msg),
            YamlError::YAMLParsingError(msg, _err) => write!(f, "{}.", msg),
            YamlError::YAMLFormatError(msg, _err) => write!(f, "{}.", msg),
        }
    }
}

impl fmt::Debug for YamlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YamlError::IoError(msg, err) => write!(f, "{}:\n {:?}", msg, err),
            YamlError::NotFound(msg, err) => write!(f, "{}:\n {:?}", msg, err),
            YamlError::YAMLParsingError(msg, err) => write!(f, "{}:\n {:?}", msg, err),
            YamlError::YAMLFormatError(msg, err) => write!(f, "{}:\n {:?}", msg, err),
        }
    }
}

impl Error for YamlError {
    fn description(&self) -> &str {
        match self {
            YamlError::IoError(msg, _err) => msg,
            YamlError::NotFound(msg, _err) => msg,
            YamlError::YAMLParsingError(msg, _err) => msg,
            YamlError::YAMLFormatError(msg, _err) => msg,
        }
    }
}

impl From<serde_yaml::Error> for YamlError {
    fn from(err: serde_yaml::Error) -> Self {
        YamlError::YAMLParsingError("Error parsing YAML file".to_string(), err)
    }
}

impl From<io::Error> for YamlError {
    fn from(err: io::Error) -> Self {
        YamlError::IoError("Error while creating / opening file".to_string(), err)
    }
}



