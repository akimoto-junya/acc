use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Test {
    pub compiler: Option<String>,
    pub compile_arg: Option<String>,
    pub command: String,
    pub command_arg: Option<String>,
    pub tle_time: u16,
    pub print_wrong_answer: bool,
}
