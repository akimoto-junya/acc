use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cookie {
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub csrf_token: String,
    pub cookies: Vec<Cookie>,
}
