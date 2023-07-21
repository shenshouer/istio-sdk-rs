use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct PortSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,
}
