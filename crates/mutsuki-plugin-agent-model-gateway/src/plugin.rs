use mutsuki_agent_protocol::*;
use mutsuki_agent_sdk::{
    AgentModelGenerateProtocol, orchestration_runner, service_result_event, unsupported_protocol,
};
use mutsuki_runtime_sdk::contracts::{RunnerResult, Task};
use mutsuki_runtime_sdk::{AsyncRunnerAdapter, PluginBuilder, RuntimeClientRef, RuntimeResult};

use crate::ModelGateway;

pub const PLUGIN_ID: &str = "mutsuki.plugin.agent.model_gateway";
pub const RUNNER_ID: &str = "mutsuki.agent.model_gateway.runner";

pub fn plugin(client: RuntimeClientRef, gateway: ModelGateway) -> PluginBuilder {
    PluginBuilder::new(PLUGIN_ID)
        .protocol::<AgentModelGenerateProtocol>()
        .runner(Box::new(runner(client, gateway)))
}

pub fn runner(client: RuntimeClientRef, gateway: ModelGateway) -> AsyncRunnerAdapter {
    let descriptor = orchestration_runner(RUNNER_ID, PLUGIN_ID)
        .accepts::<AgentModelGenerateProtocol>()
        .build();
    AsyncRunnerAdapter::new(
        descriptor,
        client,
        Box::new(move |_ctx, task| {
            let gateway = gateway.clone();
            Box::pin(async move { run_task(gateway, task).await })
        }),
    )
}

async fn run_task(gateway: ModelGateway, task: Task) -> RuntimeResult<RunnerResult> {
    match task.protocol_id.as_str() {
        AGENT_MODEL_GENERATE_PROTOCOL => service_result_event(
            PLUGIN_ID,
            &task,
            "mutsuki.agent.model.generated",
            |request: AgentModelGenerateRequest| gateway.generate(request),
        ),
        _ => Err(unsupported_protocol(PLUGIN_ID, &task)),
    }
}
