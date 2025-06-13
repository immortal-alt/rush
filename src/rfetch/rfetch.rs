use colored::*;
use sysinfo::System;
use whoami::{self, fallible};

pub fn rfetch() {
    let platform = whoami::platform();

    match platform {
        whoami::Platform::Linux => {
            let mut sys = System::new_all();
            sys.refresh_all();
            println!("{}", crate::rfetch::icons::tux.bright_blue());
            println!("{}", "=== Linux System Information ===".bold().underline());
            println!("{:<15}: {}", "OS".green(), whoami::distro());
            println!(
                "{:<15}: {}",
                "Hostname".green(),
                fallible::hostname().unwrap_or("Unknown".to_string())
            );

            println!("{}", "─".repeat(40).bright_blue());
        }
        whoami::Platform::Windows => {}
        whoami::Platform::MacOS => {}
        _ => {}
    }
}
