use chrono::{Local, Utc};

pub fn utc() {
    println!("{}", Utc::now());
}

pub fn time() {
    println!("{}", Local::now());
}
