pub const INFO: &str = "\x1b[38;5;14minfo\x1b[0m";
pub const ERROR: &str = "\x1b[38;5;9merror\x1b[0m";
pub const WARNING: &str = "\x1b[38;5;11mwarning\x1b[0m";
pub const AC: &str = "\x1b[38;5;15m\x1b[48;5;34m AC \x1b[0m";
pub const WA: &str = "\x1b[38;5;15m\x1b[48;5;214m WA \x1b[0m";
pub const RE: &str = "\x1b[38;5;15m\x1b[48;5;214m RE \x1b[0m";
pub const CE: &str = "\x1b[38;5;15m\x1b[48;5;214m CE \x1b[0m";
pub const TLE: &str = "\x1b[38;5;15m\x1b[48;5;214m TLE \x1b[0m";

pub fn get_green(text: &str) -> String {
    format!("\x1b[38;5;15m\x1b[48;5;34m {} \x1b[0m", text).to_string()
}

pub fn get_yellow(text: &str) -> String {
    format!("\x1b[38;5;15m\x1b[48;5;214m {} \x1b[0m", text).to_string()
}

pub fn get_gray(text: &str) -> String {
    format!("\x1b[38;5;15m\x1b[48;5;248m {} \x1b[0m", text).to_string()
}
