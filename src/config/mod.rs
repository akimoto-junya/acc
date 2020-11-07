use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use test::Test;
pub use language::Language;

pub mod state;
pub mod test;
pub mod language;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub contest: String,
    pub contest_task_name: Option<String>,
    pub selected_language: Option<String>,
    pub languages: HashMap<String, Language>,
}
