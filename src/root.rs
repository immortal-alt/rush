use crate::clear;
use crate::files::*;
use crate::time::*;
use crate::user::*;
use crate::utils::*;
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
        Some("wd") => println!("{}", wd()),
        Some("cd") => cd(&cmd),

        Some("say") => say(&cmd),
        Some("cat") => cat(),

        // user information
        Some("username") => println!("{}", whoami::username()),
        Some("hostname") => println!("{}", fallible::hostname().unwrap_or("Unknown".to_string())),
        Some("os") => println!("{}", whoami::distro()),
        Some("rfetch") => crate::rfetch::rfetch(),

        // time
        Some("time") => time(),
        Some("utc") => utc(),

        // service commands
        Some("help") => help_list(),
        Some("bash") => bash(&cmd),

        Some("exit") | Some("break") => process::exit(0),
        Some("clear") | Some("cls") => clear!(),

        None => {}
        Some(_) => println!("Command not found. Enter \"help\" for help."),
    }
}
