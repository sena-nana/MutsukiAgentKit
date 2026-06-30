use mutsuki_agent_protocol::*;
use mutsuki_agent_sdk::{
    AgentPromptGetProtocol, AgentPromptRenderProtocol, orchestration_runner, result_event,
    runtime_failure, task_payload,
};
use mutsuki_runtime_sdk::contracts::{RunnerResult, Task};
use mutsuki_runtime_sdk::{AsyncRunnerAdapter, PluginBuilder, RuntimeClientRef, RuntimeResult};

use crate::PromptRegistry;

pub const PLUGIN_ID: &str = "mutsuki.plugin.agent.prompt";
pub const RUNNER_ID: &str = "mutsuki.agent.prompt.runner";

pub fn plugin(client: RuntimeClientRef, registry: PromptRegistry) -> PluginBuilder {
    PluginBuilder::new(PLUGIN_ID)
        .protocol::<AgentPromptRenderProtocol>()
        .protocol::<AgentPromptGetProtocol>()
        .runner(Box::new(runner(client, registry)))
}

pub fn runner(client: RuntimeClientRef, registry: PromptRegistry) -> AsyncRunnerAdapter {
    let descriptor = orchestration_runner(RUNNER_ID, PLUGIN_ID)
        .accepts::<AgentPromptRenderProtocol>()
        .accepts::<AgentPromptGetProtocol>()
        .build();
    AsyncRunnerAdapter::new(
        descriptor,
        client,
        Box::new(move |_ctx, task| {
            let registry = registry.clone();
            Box::pin(async move { run_task(registry, task).await })
        }),
    )
}

async fn run_task(registry: PromptRegistry, task: Task) -> RuntimeResult<RunnerResult> {
    match task.protocol_id.as_str() {
        AGENT_PROMPT_RENDER_PROTOCOL => {
            let request: AgentPromptRenderRequest = task_payload(PLUGIN_ID, &task)?;
            let result = registry
                .render(request)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            result_event(task.task_id, "mutsuki.agent.prompt.rendered", result)
        }
        AGENT_PROMPT_GET_PROTOCOL => {
            let request: AgentPromptGetRequest = task_payload(PLUGIN_ID, &task)?;
            let result = registry
                .get(request)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            result_event(task.task_id, "mutsuki.agent.prompt.loaded", result)
        }
        _ => Ok(RunnerResult::completed(task.task_id)),
    }
}
