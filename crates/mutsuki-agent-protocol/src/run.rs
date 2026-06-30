use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{AgentMessage, AgentUsage};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRunStatus {
    Completed,
    WaitingApproval,
    BudgetExceeded,
    Cancelled,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentRunRequest {
    pub profile_id: String,
    pub messages: Vec<AgentMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default = "default_max_steps")]
    pub max_steps: u32,
    #[serde(default)]
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentLoopStepRequest {
    pub run: AgentRunRequest,
    pub step_index: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentStepRecord {
    pub step_index: u32,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentRunResult {
    pub status: AgentRunStatus,
    pub messages: Vec<AgentMessage>,
    #[serde(default)]
    pub steps: Vec<AgentStepRecord>,
    #[serde(default)]
    pub usage: AgentUsage,
}

fn default_max_steps() -> u32 {
    8
}
