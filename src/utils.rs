
use std::{ffi::OsString, collections::HashMap};

use crate::{lexer::Token, library::{Command, COMMAND_MAP}};

pub fn format_directory(dir: &OsString) -> String {
    format!("{}/", dir.to_str().unwrap())
}

pub fn are_valid_flags(cmd: &str, flags: &Token) -> bool {

    let valid_flags = COMMAND_MAP.get_cmd(cmd).flags;
    flags.get_value().chars().all(|flag| valid_flags.contains(flag))
}

pub fn launch_prompt() {
    print!("\n> ");
}

pub trait CommandMapOps {
    fn get_cmd(&self, cmd: &str) -> Command;
}

impl CommandMapOps for HashMap<&'static str, Command> {

    fn get_cmd(&self, cmd: &str) -> Command {
        *self.get(cmd).unwrap()
    }
}
