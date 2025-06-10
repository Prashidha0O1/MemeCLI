use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemeTemplate {
    pub name: String,
    pub ascii_art: String,
    pub placeholder: String,
}
