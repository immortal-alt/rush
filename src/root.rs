use crate::clear;
use crate::files::*;
use crate::user::*;
use std::process;
use whoami;
use whoami::fallible;

pub fn read_command(cmd: Vec<String>) {
    match cmd.first().map(|s| s.as_str()) {
        // working with the file system
        Some("cf") => cf(&cmd),
        Some("md") => md(&cmd),
        Some("rm") => rm(&cmd),
        Some("ls") => ls(&cmd),
        Some("wf") => wf(&cmd),
        Some("wd") => wd(),
        Some("cd") => cd(&cmd),
        
        Some("say") => say(&cmd),
        Some("cat") => cat(),
        
        // user information
        Some("username") => println!("{}", whoami::username()),
        Some("hostname") => println!("{}", fallible::hostname().unwrap_or("Unknown".to_string())),
        Some("os") => println!("{}", whoami::distro()),

        // service commands
        Some("exit") | Some("break") => process::exit(0),
        Some("clear") | Some("cls") => clear!(),
        _ => {}
    }
}
