use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{AgentMessage, AgentUsage};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentModelGenerateRequest {
    pub model: String,
    pub messages: Vec<AgentMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_hint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentModelGenerateResult {
    pub message: AgentMessage,
    #[serde(default)]
    pub usage: AgentUsage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentModelStreamRequest {
    pub request: AgentModelGenerateRequest,
}
