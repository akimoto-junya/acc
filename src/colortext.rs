use termion::color::{self, Bg, Fg};
use termion::style;

pub fn info() -> String {
    format!("{}info{}", Fg(color::Rgb(0, 255, 255)), style::Reset).to_string()
}

pub fn error() -> String {
    format!("{}error{}", Fg(color::Rgb(255, 0, 0)), style::Reset).to_string()
}

pub fn warning() -> String {
    format!("{}warning{}", Fg(color::Rgb(255, 255, 0)), style::Reset).to_string()
}

pub fn ac() -> String {
    format!("{}{} AC {}", Fg(color::Rgb(255, 255, 255)), Bg(color::Rgb(92, 184, 92)), style::Reset).to_string()
}

pub fn wa() -> String {
    format!("{}{} WA {}", Fg(color::Rgb(255, 255, 255)), Bg(color::Rgb(240, 173, 78)), style::Reset).to_string()
}

pub fn re() -> String {
    format!("{}{} RE {}", Fg(color::Rgb(255, 255, 255)), Bg(color::Rgb(240, 173, 78)), style::Reset).to_string()
}

pub fn ce() -> String {
    format!("{}{} CE {}", Fg(color::Rgb(255, 255, 255)), Bg(color::Rgb(240, 173, 78)), style::Reset).to_string()
}

pub fn tle() -> String {
    format!("{}{} TLE {}", Fg(color::Rgb(255, 255, 255)), Bg(color::Rgb(240, 173, 78)), style::Reset).to_string()
}
