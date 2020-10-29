use serde::{Deserialize, Serialize};

pub use test::Test;

pub mod state;
pub mod test;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub contest: Option<String>,
    pub contest_task_name: Option<String>,
    pub total_task: Option<u8>,
    pub extension: Option<String>,
    pub language_id: Option<u16>,
    pub test: Test,
}

pub trait Overridable {
    fn override_by_default(&mut self);
}
