use mutsuki_agent_protocol::{AgentMessage, AgentRunRequest};
use mutsuki_plugin_agent_loop::AgentLoop;

fn main() {
    let result = AgentLoop::default()
        .run(AgentRunRequest {
            profile_id: "example.simple".into(),
            messages: vec![AgentMessage::user("hello")],
            session_id: None,
            max_steps: 1,
            stream: false,
            model: Some("mock".into()),
            metadata: None,
            result_protocol_id: None,
            result_context: None,
        })
        .expect("agent run succeeds");

    for message in result.messages {
        println!("{:?}: {}", message.role, message.content);
    }
}
