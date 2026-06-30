use mutsuki_agent_protocol::AgentModelGenerateRequest;
use mutsuki_runtime_sdk::{AsyncRunnerContext, CallFuture};

use crate::AgentModelGenerateProtocol;

#[derive(Clone)]
pub struct ModelClient {
    ctx: AsyncRunnerContext,
}

impl ModelClient {
    pub fn new(ctx: AsyncRunnerContext) -> Self {
        Self { ctx }
    }

    pub fn generate(&self, request: AgentModelGenerateRequest) -> CallFuture {
        self.ctx.call::<AgentModelGenerateProtocol>(request)
    }
}
