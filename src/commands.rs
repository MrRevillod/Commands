
use std::{env, fs};
use std::io::BufRead;
use std::path::{Path, PathBuf};

use crate::lexer::Token;
use crate::library::OsKind;
use crate::error::ExplorerError;
use crate::history::load_history;
use crate::utils::format_directory;

pub fn exit(_: Option<Token>, _: Vec<Token>) -> Result<(), ExplorerError> {
    std::process::exit(0)
}

pub fn clear(_: Option<Token>, _: Vec<Token>) -> Result<(), ExplorerError> {
    Ok(print!("\x1B[2J\x1B[1;1H"))
}

pub fn list(flags: Option<Token>, args: Vec<Token>) -> Result<(), ExplorerError> {

    let flags = match flags {
        Some(flags) => flags.get_value(),
        None => String::new()
    };

    let mut directory: PathBuf = PathBuf::new();

    if args.len() > 1 {
        return Err(ExplorerError::InvalidNumberOfArguments)
    }

    if args.is_empty() {

        directory = match env::current_dir() {
            Ok(dir) => dir,
            Err(_) => return Err(ExplorerError::CurrentDirectory)
        }
    }

    if !directory.exists() {

        directory = PathBuf::from(args.get(0).unwrap().get_value());

        if directory.is_file() {
            return  Err(ExplorerError::CannotListAFile)
        }

        if !directory.exists() {
            return Err(ExplorerError::DirectoryDoesntExists)
        }
    }

    for entry in directory.read_dir().expect("read dir call failed") {

        if let Ok(entry) = entry {

            let file = entry.file_name();
            let mut output = file.to_str().unwrap().to_string();

            if entry.path().is_dir() {
                output = format_directory(&file);
            }
            
            if file.to_str().unwrap().starts_with(".") && !flags.contains("a") {
                continue
            }

            if !flags.contains("l") { print!("{} ", output) } else { println!("{}", output) }
        }
    }

    Ok(())
}

pub fn pwd(_: Option<Token>, _: Vec<Token>) -> Result<(), ExplorerError> {

    println!("{}", env::current_dir().unwrap().display());
    Ok(())
}

pub fn history(_: Option<Token>, _: Vec<Token>) -> Result<(), ExplorerError> {

    let history_path = Path::new("./.file_exp_history");

    if !history_path.exists() {
        load_history().map_err(|_| return ExplorerError::UnknownHistoryError)?
    }

    for line in fs::read(history_path).expect("Loading History error").lines() {
        println!("{}", line.unwrap())
    }

    Ok(())
}

pub fn change_directory(_: Option<Token>, args: Vec<Token>) -> Result<(), ExplorerError> {

    let directory = if args.is_empty() {

        let handle_env_var_error = || ExplorerError::HomeEnvVariableNotFound;

        let home = match OsKind::get_kind() {
            OsKind::Unix => env::var("HOME").map_err(|_| handle_env_var_error()),
            OsKind::Windows => env::var("USERPROFILE").map_err(|_| handle_env_var_error())
        };

        PathBuf::from(home?)
        
    } else {
        PathBuf::from(args.get(0).unwrap().get_value())
    };

    env::set_current_dir(&directory).map_err(|e| {

        match e.kind() {
            std::io::ErrorKind::NotFound => ExplorerError::DirectoryDoesntExists,
            std::io::ErrorKind::PermissionDenied => ExplorerError::PermissionDenied,
            _ => ExplorerError::GivenDirectory
        }
    })?;

    Ok(())
}

#[warn(dead_code)]
pub fn move_cmd(flags: Option<Token>, args: Vec<Token>) -> Result<(), ExplorerError> {
    Ok(())
}

#[warn(dead_code)]
pub fn copy_cmd(flags: Option<Token>, args: Vec<Token>) -> Result<(), ExplorerError> {
    Ok(())
}
