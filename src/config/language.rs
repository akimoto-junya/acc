use serde::{Deserialize, Serialize};
use crate::config::Test;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Language {
    pub extension: String,
    pub language_id: String,
    pub test: Test,
}
