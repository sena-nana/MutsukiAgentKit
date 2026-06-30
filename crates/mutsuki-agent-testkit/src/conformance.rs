use mutsuki_agent_protocol::{
    AgentMemoryQueryRequest, AgentMemoryWriteRequest, AgentMessage, AgentPromptRenderRequest,
    AgentPromptTemplate, AgentRunRequest, AgentRunStatus, AgentSessionAppendRequest,
    AgentSessionCreateRequest,
};
use mutsuki_plugin_agent_loop::AgentLoop;
use mutsuki_plugin_agent_memory_router::MemoryRouter;
use mutsuki_plugin_agent_prompt::PromptRegistry;
use mutsuki_plugin_agent_session::SessionStore;

pub fn run_basic_conformance() {
    session_round_trip();
    memory_round_trip();
    prompt_round_trip();
    agent_loop_round_trip();
}

pub fn session_round_trip() {
    let store = SessionStore::default();
    let session = store
        .create(AgentSessionCreateRequest {
            profile_id: "test.profile".into(),
            title: Some("Conformance".into()),
        })
        .expect("session can be created");
    let session = store
        .append(AgentSessionAppendRequest {
            session_id: session.session_id,
            messages: vec![AgentMessage::user("hello")],
        })
        .expect("session can append messages");
    assert_eq!(session.messages.len(), 1);
    assert_eq!(session.turn_count, 1);
}

pub fn memory_round_trip() {
    let memory = MemoryRouter::default();
    memory
        .write(AgentMemoryWriteRequest {
            text: "workspace uses Rust native plugins".into(),
            tags: vec!["architecture".into()],
            metadata: None,
        })
        .expect("memory can be written");
    let result = memory
        .query(AgentMemoryQueryRequest {
            query: "Rust plugins".into(),
            limit: 4,
            tags: Vec::new(),
        })
        .expect("memory can be queried");
    assert_eq!(result.records.len(), 1);
}

pub fn prompt_round_trip() {
    let prompts = PromptRegistry::default();
    prompts
        .register(AgentPromptTemplate {
            template_id: "hello".into(),
            body: "Hello {{name}}".into(),
            variables: vec!["name".into()],
        })
        .expect("prompt can be registered");
    let rendered = prompts
        .render(AgentPromptRenderRequest {
            template_id: "hello".into(),
            variables: [("name".to_string(), serde_json::json!("Mutsuki"))].into(),
        })
        .expect("prompt can be rendered");
    assert_eq!(rendered.text, "Hello Mutsuki");
}

pub fn agent_loop_round_trip() {
    let result = AgentLoop::default()
        .run(AgentRunRequest {
            profile_id: "test.profile".into(),
            messages: vec![AgentMessage::user("ping")],
            session_id: None,
            max_steps: 1,
            stream: false,
            model: Some("mock".into()),
            metadata: None,
        })
        .expect("agent loop can complete");
    assert_eq!(result.status, AgentRunStatus::Completed);
    assert_eq!(result.messages.len(), 2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_conformance_passes() {
        super::run_basic_conformance();
    }
}
