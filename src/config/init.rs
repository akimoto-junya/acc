use serde::{Deserialize, Serialize};
use crate::config::Overridable;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Init {
    total_task: Option<u8>,
    needs_copy_delault: Option<bool>
}

impl Overridable for Init {
    fn override_by_default(&mut self) {
        if self.total_task.is_none() {
            self.total_task = Some(6);
        }
        if self.needs_copy_delault.is_none() {
            self.needs_copy_delault = Some(false);
        }
    }
}
