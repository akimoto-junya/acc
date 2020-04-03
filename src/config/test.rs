use serde::{Deserialize, Serialize};
use crate::config::Overridable;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Test {
    pub compiler: Option<String>,
    pub compile_arg: Option<String>,
    pub command: String,
    pub command_arg: Option<String>,
    pub tle_time: Option<u16>,
    pub print_wrong_answer: Option<bool>,
}

impl Overridable for Test {
    fn override_by_default(&mut self) {
        if self.tle_time.is_none() {
            self.tle_time = Some(3000);
        }
        if self.print_wrong_answer.is_none() {
            self.print_wrong_answer = Some(true);
        }
    }
}

