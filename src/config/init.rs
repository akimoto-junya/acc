use serde::{Deserialize, Serialize};
use crate::config::Overridable;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Init {
    pub total_task: Option<u8>,
    pub extension: Option<String>,
}

impl Overridable for Init {
    fn override_by_default(&mut self) {
        if self.total_task.is_none() {
            self.total_task = Some(6);
        }
    }
}
