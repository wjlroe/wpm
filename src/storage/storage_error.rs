use rmp::*;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum StorageError {
    MissingCorrectWords(decode::ValueReadError),
    MissingIncorrectWords(decode::ValueReadError),
    MissingBackspaces(decode::ValueReadError),
    MissingWpm(decode::ValueReadError),
    MissingTime(decode::ValueReadError),
    MissingNotesLen(decode::ValueReadError),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StorageError::MissingCorrectWords(ref err) => {
                write!(f, "ValueReadError error: {}", err)
            }
            StorageError::MissingIncorrectWords(ref err) => {
                write!(f, "ValueReadError error: {}", err)
            }
            StorageError::MissingBackspaces(ref err) => write!(f, "ValueReadError error: {}", err),
            StorageError::MissingWpm(ref err) => write!(f, "ValueReadError error: {}", err),
            StorageError::MissingTime(ref err) => write!(f, "ValueReadError error: {}", err),
            StorageError::MissingNotesLen(ref err) => write!(f, "ValueReadError error: {}", err),
        }
    }
}

impl error::Error for StorageError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            StorageError::MissingCorrectWords(ref err) => Some(err),
            StorageError::MissingIncorrectWords(ref err) => Some(err),
            StorageError::MissingBackspaces(ref err) => Some(err),
            StorageError::MissingWpm(ref err) => Some(err),
            StorageError::MissingTime(ref err) => Some(err),
            StorageError::MissingNotesLen(ref err) => Some(err),
        }
    }
}
