use std::io;
use std::process::Command;

pub fn say(cmd: &[String]) {
    let mut result: String = String::new();
    for (i, to_say) in cmd[1..].iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(to_say);
    }
    println!("{}", result);
}

pub fn cat() {
    let cat = r#"    /\_____/\
   /  o   o  \
  ( ==  ^  == )
   )         (
  (           )
 ( (  )   (  ) )
(__(__)___(__)__)"#;

    println!("{}", cat);
}

pub fn bash(cmd: &[String]) {
    if cmd.len() < 2 {
        return;
    }
    let command = &cmd[1];
    let args = &cmd[2..];

    let mut child = Command::new(command)
        .args(args)
        .stdout(io::stdout())
        .stderr(io::stderr())
        .spawn();

    match child {
        Ok(ref mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            println!("Error: failed to execute command '{}': {}", command, e);
        }
    }
}
