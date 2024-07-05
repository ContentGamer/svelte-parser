#![allow(non_snake_case)]
#![allow(dead_code)]

fn base(color: &str, level: &str, message: &str) -> String {
    format!("\x1b[{}m{}: \x1b[0m{}", color, level, message)
}

pub fn DEBUG(message: &str) -> () {
    println!("{}", base("90", "DEBUG", message));
}

pub fn INFO(message: &str) -> () {
    println!("{}", base("34", "INFO", message));
}

pub fn WARN(message: &str) -> () {
    println!("{}", base("33", "WARN", message));
}

pub fn ERROR(message: &str) -> () {
    println!("{}", base("31", "ERROR", message));
}

pub fn CRITICAL(message: &str) -> () {
    panic!("{}", base("1;31", "CRITICAL", message));
}
