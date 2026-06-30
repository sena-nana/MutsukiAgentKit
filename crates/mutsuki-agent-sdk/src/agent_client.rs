use mutsuki_agent_protocol::{AgentMessage, AgentRunRequest};
use mutsuki_runtime_sdk::{AsyncRunnerContext, CallFuture};

use crate::AgentRunProtocol;

#[derive(Clone)]
pub struct AgentClient {
    ctx: AsyncRunnerContext,
}

impl AgentClient {
    pub fn new(ctx: AsyncRunnerContext) -> Self {
        Self { ctx }
    }

    pub fn run(&self, request: AgentRunRequest) -> CallFuture {
        self.ctx.call::<AgentRunProtocol>(request)
    }

    pub fn run_text(&self, profile_id: impl Into<String>, text: impl Into<String>) -> AgentRunCall {
        AgentRunCall {
            ctx: self.ctx.clone(),
            profile_id: profile_id.into(),
            messages: vec![AgentMessage::user(text)],
            session_id: None,
            max_steps: 8,
            stream: false,
            model: None,
        }
    }
}

pub struct AgentRunCall {
    ctx: AsyncRunnerContext,
    profile_id: String,
    messages: Vec<AgentMessage>,
    session_id: Option<String>,
    max_steps: u32,
    stream: bool,
    model: Option<String>,
}

impl AgentRunCall {
    pub fn profile(mut self, profile_id: impl Into<String>) -> Self {
        self.profile_id = profile_id.into();
        self
    }

    pub fn session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn max_steps(mut self, max_steps: u32) -> Self {
        self.max_steps = max_steps;
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = stream;
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn call(self) -> CallFuture {
        self.ctx.call::<AgentRunProtocol>(AgentRunRequest {
            profile_id: self.profile_id,
            messages: self.messages,
            session_id: self.session_id,
            max_steps: self.max_steps,
            stream: self.stream,
            model: self.model,
            metadata: None,
        })
    }
}
