#[macro_export]
macro_rules! clear {
    () => {
        print!("\x1B[2J\x1B[1;1H")
    };
}

#[macro_export]
macro_rules! scan_cmd {
    () => {{
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }};
}
