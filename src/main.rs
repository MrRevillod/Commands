
mod lexer;
mod utils;
mod error;
mod library;
mod history;
mod commands;

use lexer::Token;
use error::ExplorerError;
use library::{COMMAND_MAP, COMMAND_SET};
use history::{load_history, save_command};
use utils::{are_valid_flags, launch_prompt, CommandMapOps};
use std::{error::Error, io::{stdin, stdout, BufRead, Write}};

fn execute(cmd: Token, flags: Option<Token>, args: Vec<Token>) -> Result<(), ExplorerError> {

    let cmd = cmd.get_value();

    if !COMMAND_SET.contains(&cmd.as_str()) {
        return Err(ExplorerError::CommandNotFound)
    }
    
    let cmd_info = COMMAND_MAP.get_cmd(&cmd);

    if let Some(flag) = &flags {

        if !cmd_info.accepts_flags {
            return Err(ExplorerError::CommandDoesNotAcceptFlags)
        }

        if !are_valid_flags(&cmd, flag) {
            return Err(ExplorerError::InvalidFlag)
        }
    }

    if args.len() != 0 && !cmd_info.accepts_args {
        return Err(ExplorerError::CommandDoesNotAcceptArguments)
    }

    if !cmd_info.optional_args && args.len() != cmd_info.number_of_args as usize {
        return Err(ExplorerError::InvalidNumberOfArguments)
    }

    let command_function = cmd_info.function;
    command_function(flags, args)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {

    load_history()?;

    loop {

        let mut input = String::new();
    
        launch_prompt();
        stdout().flush().unwrap();
        stdin().lock().read_line(&mut input).unwrap();

        input = input.trim().to_string();
        save_command(&input)?;

        let mut tokens = lexer::tokenize(&input);
        let command = match lexer::parse(&mut tokens) {
            
            Ok(command) => command,
            Err(e) => {
                eprintln!("{}", e); continue
            }
        };

        if let Err(e) = execute(command.0, command.1, command.2) {

            dbg!(&e);

            eprintln!("{}", e);
        }
    }
}
