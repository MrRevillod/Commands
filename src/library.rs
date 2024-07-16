
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

use crate::commands::*;
use crate::lexer::Token;
use crate::error::ExplorerError;

pub enum OsKind {
    Unix,
    Windows,
}

impl OsKind {
    pub fn get_kind() -> Self {
        match std::env::consts::OS {
            "linux" | "macos" => OsKind::Unix,
            "windows" => OsKind::Windows,
            _ => panic!("Unsupported OS")
        }
    }
}

type CommandFunction = fn(flags: Option<Token>, args: Vec<Token>) -> Result<(), ExplorerError>;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Command {
    pub name: &'static str,
    pub flags: &'static str,
    pub accepts_flags: bool,
    pub accepts_args: bool,
    pub number_of_args: u8,
    pub optional_args: bool,
    pub help: &'static str,
    pub function: CommandFunction
}

lazy_static!(

    pub static ref COMMAND_SET: HashSet<&'static str> = HashSet::from([
        "ls", "pwd", "clear", "exit", "history", "cd"
    ]);

    pub static ref COMMAND_MAP: HashMap<&'static str, Command> = HashMap::from([
        
        ("ls", Command {
            name: "ls",
            flags: "al",
            accepts_flags: true,
            accepts_args: true,
            number_of_args: 1,
            optional_args: true,
            help: "List directory contents",
            function: list
        }),
        ("pwd", Command {
            name: "pwd",
            flags: "",
            accepts_flags: false,
            accepts_args: false,
            number_of_args: 0,
            optional_args: false,
            help: "Print working directory",
            function: pwd
        }),
        ("clear", Command {
            name: "clear",
            flags: "",
            accepts_flags: false,
            accepts_args: false,
            number_of_args: 0,
            optional_args: false,
            help: "Clear the screen",
            function: clear
        }),
        ("exit", Command {
            name: "exit",
            flags: "",
            accepts_flags: false,
            accepts_args: false,
            number_of_args: 0,
            optional_args: false,
            help: "Exit the shell",
            function: exit
        }),
        ("history", Command {
            name: "history",
            flags: "",
            accepts_flags: false,
            accepts_args: false,
            number_of_args: 0,
            optional_args: false,
            help: "Show command history",
            function: history
        }),
        ("cd", Command {
            name: "cd",
            flags: "",
            accepts_flags: false,
            accepts_args: true,
            number_of_args: 1,
            optional_args: true,
            help: "Change directory",
            function: change_directory
        })
    ]);
);