use serde::{Deserialize, Serialize};
use crate::config::Overridable;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Test {
    compiler: Option<String>,
    compile_arg: Option<String>,
    needs_put: Option<bool>,
    command: String,
    command_arg: Option<String>,
    tle_time: Option<u16>,
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

