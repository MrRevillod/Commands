
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ExplorerError {
    CommandNotFound,
    InvalidFlag,
    CommandDoesNotAcceptFlags,
    InvalidNumberOfArguments,
    CommandDoesNotAcceptArguments,

    CurrentDirectory,
    GivenDirectory,
    CannotListAFile,
    DirectoryDoesntExists,
    UnknownHistoryError,
    PermissionDenied,

    HomeEnvVariableNotFound,
}

impl fmt::Display for ExplorerError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match *self {
            ExplorerError::CommandNotFound => write!(f, "Command not found"),
            ExplorerError::InvalidFlag => write!(f, "Invalid flag"),
            ExplorerError::CommandDoesNotAcceptFlags => write!(f, "This command does not accept flags. Use help for more information."),
            ExplorerError::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
            ExplorerError::CommandDoesNotAcceptArguments => write!(f, "Command does not accept arguments"),
            ExplorerError::CurrentDirectory => write!(f, "There is an error with the current directory. Please try again."),
            ExplorerError::GivenDirectory => write!(f, "There is an error with the given directory. Please try again."),
            ExplorerError::CannotListAFile => write!(f, "You cannot list a file."),
            ExplorerError::DirectoryDoesntExists => write!(f, "The given directory doesn't exists"),
            ExplorerError::UnknownHistoryError => write!(f, "Error happened with the history"),
            ExplorerError::PermissionDenied => write!(f, "Permission denied"),
            ExplorerError::HomeEnvVariableNotFound => write!(f, "Home environment variable not found"),
        }
    }
}

impl Error for ExplorerError {}

#[derive(Debug)]
pub enum ParseError {
    InvalidFlagPosition,
    InvalidArgumentPosition,
}

impl fmt::Display for ParseError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match *self {
            ParseError::InvalidFlagPosition => write!(f, "Invalid flag position"),
            ParseError::InvalidArgumentPosition => write!(f, "Invalid argument position"),
        }
    }
}

impl Error for ParseError {}
