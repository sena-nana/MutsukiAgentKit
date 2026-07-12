use mutsuki_agent_protocol::{
    AGENT_LOOP_STEP_PROTOCOL, AGENT_MODEL_GENERATE_PROTOCOL, AGENT_TOOL_EXECUTE_PROTOCOL,
    AgentError, AgentLoopStepRequest, AgentMessage, AgentModelGenerateRequest, AgentResult,
    AgentRole, AgentRunRequest, AgentRunResult, AgentRunStatus, AgentStepRecord,
    AgentToolExecuteRequest, AgentUsage,
};
use mutsuki_runtime_sdk::contracts::Task;
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

    pub fn plan_tasks(&self, request: &AgentRunRequest, parent: &Task) -> AgentResult<Vec<Task>> {
        if request.max_steps == 0 {
            return Ok(Vec::new());
        }
        let model = request
            .model
            .clone()
            .unwrap_or_else(|| self.default_model.clone());
        let mut tasks = vec![child_task(
            parent,
            "model",
            AGENT_MODEL_GENERATE_PROTOCOL,
            serde_json::to_value(AgentModelGenerateRequest {
                model,
                messages: request.messages.clone(),
                temperature: None,
                max_output_tokens: None,
                provider_hint: None,
                metadata: request.metadata.clone(),
                result_protocol_id: request.result_protocol_id.clone(),
                result_context: request.result_context.clone(),
                session_id: request.session_id.clone(),
            })
            .map_err(|error| AgentError::invalid_input(error.to_string()))?,
        )];
        if let Some(tool) = request
            .metadata
            .as_ref()
            .and_then(|metadata| metadata.get("tool"))
        {
            let name = tool
                .get("name")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| AgentError::invalid_input("metadata.tool.name is required"))?;
            tasks.push(child_task(
                parent,
                "tool",
                AGENT_TOOL_EXECUTE_PROTOCOL,
                serde_json::to_value(AgentToolExecuteRequest {
                    name: name.into(),
                    input: tool.get("input").cloned().unwrap_or_default(),
                    session_id: request.session_id.clone(),
                })
                .map_err(|error| AgentError::invalid_input(error.to_string()))?,
            ));
        }
        tasks.push(child_task(
            parent,
            "result",
            AGENT_LOOP_STEP_PROTOCOL,
            serde_json::to_value(AgentLoopStepRequest {
                run: request.clone(),
                step_index: 0,
            })
            .map_err(|error| AgentError::invalid_input(error.to_string()))?,
        ));
        Ok(tasks)
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

fn child_task(parent: &Task, suffix: &str, protocol_id: &str, payload: serde_json::Value) -> Task {
    let mut task = Task::new(format!("{}:{suffix}", parent.task_id), protocol_id, payload);
    task.trace_id = parent.trace_id.clone();
    task.correlation_id = parent.correlation_id.clone();
    task.registry_generation = parent.registry_generation;
    task
}
