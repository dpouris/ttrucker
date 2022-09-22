use crate::printil;
use std::io::Result;

pub fn get_input() -> Result<String> {
    let mut buffer = String::new();

    printil!("> ");
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

pub fn clear_term() {
    // print!("\u{001b}c");
    print!("\x1b[2J\x1b[0;0H");
}

pub fn hide_cursor() {
    print!("\x1b[?25l")
}

pub fn show_cursor() {
    print!("\x1b[?25h")
}
