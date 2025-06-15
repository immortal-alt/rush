// use colored::*;
use std::env;
use std::fs::{self, File};
use std::path::Path;
use terminal_size::{Width, terminal_size};

pub fn cf(cmd: &[String]) {
    if cmd.len() != 2 {
        println!("Error: Missing file name");
        println!("Usage: cf <filename>");
        return;
    }

    let file_path = Path::new(&cmd[1]);

    if file_path.exists() && file_path.is_file() {
        println!("File {} already exists", &cmd[1]);
        return;
    }

    match File::create(&cmd[1]) {
        Ok(_) => {}
        Err(e) => println!("Failed to create file: {}", e),
    }
}

pub fn md(cmd: &[String]) {
    if cmd.len() != 2 {
        println!("Error: Missing directory name");
        println!("Usage: cf <directory>");
        return;
    }

    let dir_path = Path::new(&cmd[1]);

    if dir_path.exists() && dir_path.is_dir() {
        println!("Directory {} already exists", &cmd[1]);
        return;
    }

    match fs::create_dir_all(&cmd[1]) {
        Ok(_) => {}
        Err(e) => println!("Failed to create directory: {}", e),
    }
}

pub fn rm(cmd: &[String]) {
    if cmd.len() != 2 {
        println!("Error: Missing target name");
        println!("Usage: rm <directory or file>");
        return;
    }

    let target_path = Path::new(&cmd[1]);

    if !target_path.exists() {
        println!("Target is not exists");
        return;
    }

    if target_path.is_file() {
        match fs::remove_file(&cmd[1]) {
            Ok(_) => {}
            Err(e) => {
                println!("Error in removing file: {}", e);
                return;
            }
        }
    } else if target_path.is_dir() {
        match fs::remove_dir_all(&cmd[1]) {
            Ok(_) => {}
            Err(e) => {
                println!("Error in removing directory: {}", e);
                return;
            }
        }
    }
}

pub fn ls(cmd: &[String]) {
    let path = if cmd.len() > 1 { &cmd[1] } else { "." };

    match fs::read_dir(path) {
        Ok(entries) => {
            let mut file_names = Vec::new();

            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        let name = dir_entry.file_name();
                        file_names.push(name.to_string_lossy().into_owned());
                    }
                    Err(e) => println!("Error reading directory entry: {}", e),
                }
            }

            if file_names.is_empty() {
                return;
            }

            let terminal_width = match terminal_size() {
                Some((Width(w), _)) => w as usize,
                None => 80,
            };

            let max_len = file_names.iter().map(|s| s.len()).max().unwrap_or(0);
            let padding = 2;
            let column_width = max_len + padding;

            let columns = std::cmp::max(1, terminal_width / column_width);

            for (i, name) in file_names.iter().enumerate() {
                if i % columns == 0 && i != 0 {
                    println!();
                }

                let path = Path::new(name);
                let padded_name = format!("{:width$}", name, width = column_width);

                if path.is_file() {
                    print!("📄 {}", padded_name);
                } else if path.is_dir() {
                    use ansi_term::Colour::Blue;
                    print!("📁 {}", Blue.paint(padded_name));
                } else {
                    print!("   {}", padded_name);
                }
            }

            println!();
        }
        Err(e) => println!("ls: cannot access '{}': {}", path, e),
    }
}

pub fn wf(cmd: &[String]) {
    if cmd.len() != 3 {
        println!("Usage: wf <file for writing> <data to write>");
        return;
    }
    let file = Path::new(&cmd[1]);
    let data = &cmd[2];

    match fs::write(file, data) {
        Ok(_) => {}
        Err(e) => println!("Error writing data to a file: {}", e),
    }
}

pub fn wd() -> String {
    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => "?".to_string(),
    }
}

pub fn cd(cmd: &[String]) {
    if cmd.len() != 2 {
        println!("Usage: cd <path>");
        return;
    }
    let _ = match env::set_current_dir(&cmd[1]) {
        Ok(_) => {}
        Err(e) => println!("Error in switching directory: {}", e),
    };
}
