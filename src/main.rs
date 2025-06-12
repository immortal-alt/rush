mod files;
mod red;
mod root;
mod root_macros;
mod user;
mod utils;

use colored::*;

fn main() {

    println!("Welcome to {}", "rush.".red());
    loop {
        print!("{}!{:?}{}", whoami::username().bold(), files::wd(),  "= ".red());
        let input: Vec<String> = scan_cmd!();
        root::read_command(input);
    }
}


