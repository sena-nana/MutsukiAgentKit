use mutsuki_agent_protocol::*;
use mutsuki_agent_sdk::{
    AgentContextBuildProtocol, orchestration_runner, result_event, runtime_failure, task_payload,
};
use mutsuki_runtime_sdk::contracts::{RunnerResult, Task};
use mutsuki_runtime_sdk::{AsyncRunnerAdapter, PluginBuilder, RuntimeClientRef, RuntimeResult};

use crate::ContextBuilder;

pub const PLUGIN_ID: &str = "mutsuki.plugin.agent.context";
pub const RUNNER_ID: &str = "mutsuki.agent.context.runner";

pub fn plugin(client: RuntimeClientRef, builder: ContextBuilder) -> PluginBuilder {
    PluginBuilder::new(PLUGIN_ID)
        .protocol::<AgentContextBuildProtocol>()
        .runner(Box::new(runner(client, builder)))
}

pub fn runner(client: RuntimeClientRef, builder: ContextBuilder) -> AsyncRunnerAdapter {
    let descriptor = orchestration_runner(RUNNER_ID, PLUGIN_ID)
        .accepts::<AgentContextBuildProtocol>()
        .build();
    AsyncRunnerAdapter::new(
        descriptor,
        client,
        Box::new(move |_ctx, task| {
            let builder = builder.clone();
            Box::pin(async move { run_task(builder, task).await })
        }),
    )
}

async fn run_task(builder: ContextBuilder, task: Task) -> RuntimeResult<RunnerResult> {
    match task.protocol_id.as_str() {
        AGENT_CONTEXT_BUILD_PROTOCOL => {
            let request: AgentContextBuildRequest = task_payload(PLUGIN_ID, &task)?;
            let result = builder
                .build(request)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            result_event(task.task_id, "mutsuki.agent.context.built", result)
        }
        _ => Ok(RunnerResult::completed(task.task_id)),
    }
}
