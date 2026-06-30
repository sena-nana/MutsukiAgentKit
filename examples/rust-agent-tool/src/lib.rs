use mutsuki_agent_protocol::AgentResult;
use mutsuki_agent_sdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct EchoInput {
    pub text: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct EchoOutput {
    pub text: String,
}

#[agent_tool(
    name = "example.echo",
    target = "example.echo/run@1",
    description = "Echo text for Agent examples",
    side_effect = "none",
    requires_approval = false,
    permissions = []
)]
pub async fn echo_tool(_ctx: AgentToolContext, input: EchoInput) -> AgentResult<EchoOutput> {
    Ok(EchoOutput { text: input.text })
}
