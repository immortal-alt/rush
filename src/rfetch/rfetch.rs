use colored::*;
use sysinfo::System;
use whoami::{self, fallible};

pub fn rfetch() {
    let platform = whoami::platform();

    match platform {
        whoami::Platform::Linux => {
            let mut sys = System::new_all();
            sys.refresh_all();
            println!("{}", crate::rfetch::icons::TUX.bright_red());
            println!("{}", "=== System Information ===".bold().underline());
            println!("{:<15}: {}", "OS".green(), whoami::distro());
            println!(
                "{:<15}: {}",
                "Hostname".green(),
                fallible::hostname().unwrap_or("Unknown".to_string())
            );

            println!("{}", "─".repeat(40).bright_red());
        }
        whoami::Platform::Windows => {
            let mut sys = System::new_all();
            sys.refresh_all();
        }
        whoami::Platform::MacOS => {}
        _ => {}
    }
}
