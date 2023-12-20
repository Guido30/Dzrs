use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DiscordLogin {
    pub user_id: String,
    pub token: String,
    pub user_settings: HashMap<String, String>,
}
