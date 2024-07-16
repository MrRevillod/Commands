
use std::{fs, path::Path};

pub fn load_history() -> Result<(), std::io::Error> {

    let history_path = Path::new("./.file_exp_history");

    if !history_path.exists() || !history_path.is_file() {
        fs::write(history_path, "")?;
    }

    Ok(())
}

pub fn save_command(cmd: &String) -> Result<(), std::io::Error> {

    let history_path = Path::new("./.file_exp_history");
    
    if !history_path.exists() || !history_path.is_file() {
        load_history()?;
    }

    let mut buffer = fs::read_to_string(history_path)?;
    let last_index = buffer.split("\n").collect::<Vec<&str>>().len() - 1;
    
    buffer.push_str(&format!("{}. {}\n", last_index, &cmd));
    fs::write(history_path, buffer)?;

    Ok(())
}