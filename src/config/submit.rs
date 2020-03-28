use serde::{Deserialize, Serialize};
use crate::config::Overridable;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Submit {
    pub needs_test: Option<bool>,
}

impl Overridable for Submit {
    fn override_by_default(&mut self) {
        if self.needs_test.is_none() {
            self.needs_test = Some(false);
        }
    }
}
