use mutsuki_agent_protocol::*;
use mutsuki_agent_sdk::{
    AgentModelGenerateProtocol, AgentModelStreamProtocol, orchestration_runner, result_event,
    runtime_failure, task_payload,
};
use mutsuki_runtime_sdk::contracts::{RunnerResult, Task};
use mutsuki_runtime_sdk::{AsyncRunnerAdapter, PluginBuilder, RuntimeClientRef, RuntimeResult};

use crate::ModelGateway;

pub const PLUGIN_ID: &str = "mutsuki.plugin.agent.model_gateway";
pub const RUNNER_ID: &str = "mutsuki.agent.model_gateway.runner";

pub fn plugin(client: RuntimeClientRef, gateway: ModelGateway) -> PluginBuilder {
    PluginBuilder::new(PLUGIN_ID)
        .protocol::<AgentModelGenerateProtocol>()
        .protocol::<AgentModelStreamProtocol>()
        .runner(Box::new(runner(client, gateway)))
}

pub fn runner(client: RuntimeClientRef, gateway: ModelGateway) -> AsyncRunnerAdapter {
    let descriptor = orchestration_runner(RUNNER_ID, PLUGIN_ID)
        .accepts::<AgentModelGenerateProtocol>()
        .accepts::<AgentModelStreamProtocol>()
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
        AGENT_MODEL_GENERATE_PROTOCOL => {
            let request: AgentModelGenerateRequest = task_payload(PLUGIN_ID, &task)?;
            let result = gateway
                .generate(request)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            result_event(task.task_id, "mutsuki.agent.model.generated", result)
        }
        AGENT_MODEL_STREAM_PROTOCOL => {
            let request: AgentModelStreamRequest = task_payload(PLUGIN_ID, &task)?;
            let result = gateway
                .generate(request.request)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            result_event(task.task_id, "mutsuki.agent.model.stream.completed", result)
        }
        _ => Ok(RunnerResult::completed(task.task_id)),
    }
}
