use mutsuki_agent_protocol::{
    AgentError, AgentLoopStepRequest, AgentMessage, AgentResult, AgentRole, AgentRunRequest,
    AgentRunResult, AgentRunStatus, AgentStepRecord, AgentUsage,
};
use serde_json::json;

#[derive(Clone)]
pub struct AgentLoop {
    default_model: String,
}

impl Default for AgentLoop {
    fn default() -> Self {
        Self {
            default_model: "mock".into(),
        }
    }
}

impl AgentLoop {
    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = model.into();
        self
    }

    pub fn run(&self, request: AgentRunRequest) -> AgentResult<AgentRunResult> {
        if request.profile_id.trim().is_empty() {
            return Err(AgentError::invalid_input("profile_id is required"));
        }
        if request.max_steps == 0 {
            return Ok(AgentRunResult {
                status: AgentRunStatus::BudgetExceeded,
                messages: request.messages,
                steps: Vec::new(),
                usage: AgentUsage::default(),
            });
        }

        let mut messages = request.messages;
        let reply = self.generate_reply(&messages, request.model.as_deref());
        let usage = AgentUsage {
            input_tokens: messages
                .iter()
                .map(|message| message.content.len() as u64)
                .sum(),
            output_tokens: reply.content.len() as u64,
            total_tokens: messages
                .iter()
                .map(|message| message.content.len() as u64)
                .sum::<u64>()
                + reply.content.len() as u64,
        };
        messages.push(reply);
        Ok(AgentRunResult {
            status: AgentRunStatus::Completed,
            messages,
            steps: vec![AgentStepRecord {
                step_index: 0,
                kind: "model_generate".into(),
                detail: Some(json!({
                    "model": request.model.unwrap_or_else(|| self.default_model.clone()),
                    "profile_id": request.profile_id,
                })),
            }],
            usage,
        })
    }

    pub fn step(&self, request: AgentLoopStepRequest) -> AgentResult<AgentRunResult> {
        if request.step_index >= request.run.max_steps {
            return Ok(AgentRunResult {
                status: AgentRunStatus::BudgetExceeded,
                messages: request.run.messages,
                steps: Vec::new(),
                usage: AgentUsage::default(),
            });
        }
        self.run(request.run)
    }

    fn generate_reply(&self, messages: &[AgentMessage], model: Option<&str>) -> AgentMessage {
        let text = messages
            .iter()
            .rev()
            .find(|message| message.role == AgentRole::User)
            .map(|message| message.content.as_str())
            .unwrap_or("");
        let model = model.unwrap_or(&self.default_model);
        let content = if text.is_empty() {
            format!("{model}: ready")
        } else {
            format!("{model}: {text}")
        };
        AgentMessage::assistant(content)
    }
}
