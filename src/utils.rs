use colored::*;
// use std::fs;

// pub fn load_file(filename: &str) -> std::io::Result<String> {
//     match fs::metadata(filename) {
//         Ok(meta) if meta.is_dir() => Err(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             "Path is a directory",
//         )),
//         _ => fs::read_to_string(filename),
//     }
// }

pub fn help_list() {
    println!(
        "\n{}",
        "Rush Shell - A fast Rust-powered terminal"
            .bold()
            .underline()
    );
    println!("{}", "Available commands:".bold());

    let commands = vec![
        ("cf <file>", "Create a new file"),
        ("md <dir>", "Create a new directory"),
        ("rm <path>", "Remove a file/directory"),
        ("ls [dir]", "List directory contents (with icons/colors)"),
        ("wf <file>", "Write text to a file"),
        ("wd", "Show current working directory"),
        ("cd <dir>", "Change directory"),
        // Fun utilities
        ("say <text>", "Print text to terminal"),
        ("cat", "Draw an ASCII cat"),
        // System info
        ("username", "Display current username"),
        ("hostname", "Display system hostname"),
        ("os", "Show operating system info"),
        ("time", "Show local time"),
        ("utc", "Show utc time"),
        // Shell controls
        ("help", "Show this help message"),
        ("bash <bash command>", "Execute bash command"),
        ("clear/cls", "Clear the terminal screen"),
        ("exit/break", "Exit the shell"),
        ("In dev:", ""),
        ("red", "text editor"),
        ("rfetch", "rush fetch"),
        ("reload", "update config settings"),
    ];

    let max_cmd_len = commands.iter().map(|(cmd, _)| cmd.len()).max().unwrap_or(0) + 4;

    for (cmd, desc) in commands {
        println!(
            "  {:<width$} {}",
            cmd.green().bold(),
            desc,
            width = max_cmd_len
        );
    }
    println!("\nType commands and press Enter. Press Ctrl+C to interrupt operations.");
    println!(
        "Source code: {}",
        "https://github.com/immortal-alt/rush".red()
    );
}
