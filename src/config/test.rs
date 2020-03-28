use serde::{Deserialize, Serialize};
use crate::config::Overridable;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Test {
    pub compiler: Option<String>,
    pub compile_arg: Option<String>,
    pub needs_put: Option<bool>,
    pub command: String,
    pub command_arg: Option<String>,
    pub tle_time: Option<u16>,
}

impl Overridable for Test {
    fn override_by_default(&mut self) {
        if self.needs_put.is_none() {
            self.needs_put = Some(true);
        }
        if self.tle_time.is_none() {
            self.tle_time = Some(2000);
        }
    }
}

