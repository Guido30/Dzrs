use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotificationAdd<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub origin: &'a str,
    pub msg: &'a str,
}
