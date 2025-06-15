mod files;
mod rconfig;
mod rfetch;
mod root;
mod root_macros;
mod time;
mod user;
mod utils;

use colored::*;
use std::{
    // env,
    io::Write,
    // path::{Path, PathBuf},
};

// fn shorten_path(path: &Path) -> String {
//     match env::var("HOME") {
//         Ok(home_dir) => {
//             let home_path = PathBuf::from(&home_dir);
//             if path.starts_with(&home_path) {
//                 let mut shortened = String::from("~");
//                 if let Some(relative) = path.strip_prefix(&home_path).ok() {
//                     shortened.push_str("/");
//                     shortened.push_str(&relative.to_string_lossy());
//                 }
//                 shortened
//             } else {
//                 path.to_string_lossy().into_owned()
//             }
//         }
//         Err(_) => path.to_string_lossy().into_owned(),
//     }
// }

fn print_prompt() {
    let username = whoami::username();
    let current_dir = files::wd();

    print!("{}!{}{} ", username.bold(), current_dir.dimmed(), "=".red());

    std::io::stdout().flush().unwrap();
}

fn main() {
    println!("Welcome to {}", "rush.".red());
    loop {
        print_prompt();
        let input: Vec<String> = scan_cmd!();
        root::read_command(input);
    }
}
