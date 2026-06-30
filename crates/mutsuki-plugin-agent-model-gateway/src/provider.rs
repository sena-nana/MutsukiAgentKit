use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use mutsuki_agent_protocol::{
    AgentError, AgentMessage, AgentModelGenerateRequest, AgentModelGenerateResult, AgentResult,
    AgentRole, AgentUsage,
};

pub trait ModelProvider: Send + Sync {
    fn provider_id(&self) -> &str;
    fn generate(&self, request: AgentModelGenerateRequest)
    -> AgentResult<AgentModelGenerateResult>;
}

#[derive(Clone)]
pub struct ModelGateway {
    default_provider: String,
    providers: Arc<Mutex<BTreeMap<String, Arc<dyn ModelProvider>>>>,
}

impl Default for ModelGateway {
    fn default() -> Self {
        let gateway = Self {
            default_provider: "mock".into(),
            providers: Arc::new(Mutex::new(BTreeMap::new())),
        };
        gateway.register(Arc::new(MockModelProvider::default()));
        gateway
    }
}

impl ModelGateway {
    pub fn register(&self, provider: Arc<dyn ModelProvider>) {
        self.providers
            .lock()
            .expect("model gateway mutex poisoned")
            .insert(provider.provider_id().to_string(), provider);
    }

    pub fn generate(
        &self,
        request: AgentModelGenerateRequest,
    ) -> AgentResult<AgentModelGenerateResult> {
        let provider_id = request
            .provider_hint
            .clone()
            .unwrap_or_else(|| self.default_provider.clone());
        let provider = self
            .providers
            .lock()
            .expect("model gateway mutex poisoned")
            .get(&provider_id)
            .cloned()
            .ok_or_else(|| {
                AgentError::provider_unavailable(format!(
                    "model provider `{provider_id}` not registered"
                ))
            })?;
        provider.generate(request)
    }
}

#[derive(Default)]
pub struct MockModelProvider;

impl ModelProvider for MockModelProvider {
    fn provider_id(&self) -> &str {
        "mock"
    }

    fn generate(
        &self,
        request: AgentModelGenerateRequest,
    ) -> AgentResult<AgentModelGenerateResult> {
        let last_user = request
            .messages
            .iter()
            .rev()
            .find(|message| message.role == AgentRole::User)
            .map(|message| message.content.as_str())
            .unwrap_or("");
        let content = if last_user.is_empty() {
            "No user message provided.".to_string()
        } else {
            format!("Echo: {last_user}")
        };
        Ok(AgentModelGenerateResult {
            message: AgentMessage::assistant(content),
            usage: AgentUsage {
                input_tokens: request
                    .messages
                    .iter()
                    .map(|message| message.content.len() as u64)
                    .sum(),
                output_tokens: last_user.len() as u64 + 6,
                total_tokens: request
                    .messages
                    .iter()
                    .map(|message| message.content.len() as u64)
                    .sum::<u64>()
                    + last_user.len() as u64
                    + 6,
            },
            raw: None,
        })
    }
}
