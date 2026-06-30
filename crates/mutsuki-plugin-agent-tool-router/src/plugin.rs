use mutsuki_agent_protocol::*;
use mutsuki_agent_sdk::{
    AgentToolExecuteProtocol, AgentToolListProtocol, orchestration_runner, result_event,
    runtime_failure, service_result_event, task_payload, unsupported_protocol,
};
use mutsuki_runtime_sdk::contracts::{RunnerResult, Task, TaskOutcome};
use mutsuki_runtime_sdk::{
    AsyncRunnerAdapter, AsyncRunnerContext, PluginBuilder, RuntimeClientRef, RuntimeResult,
};
use serde_json::json;

use crate::ToolRegistry;

pub const PLUGIN_ID: &str = "mutsuki.plugin.agent.tool_router";
pub const RUNNER_ID: &str = "mutsuki.agent.tool_router.runner";

pub fn plugin(client: RuntimeClientRef, registry: ToolRegistry) -> PluginBuilder {
    PluginBuilder::new(PLUGIN_ID)
        .protocol::<AgentToolListProtocol>()
        .protocol::<AgentToolExecuteProtocol>()
        .runner(Box::new(runner(client, registry)))
}

pub fn runner(client: RuntimeClientRef, registry: ToolRegistry) -> AsyncRunnerAdapter {
    let descriptor = orchestration_runner(RUNNER_ID, PLUGIN_ID)
        .accepts::<AgentToolListProtocol>()
        .accepts::<AgentToolExecuteProtocol>()
        .build();
    AsyncRunnerAdapter::new(
        descriptor,
        client,
        Box::new(move |ctx, task| {
            let registry = registry.clone();
            Box::pin(async move { run_task(registry, ctx, task).await })
        }),
    )
}

async fn run_task(
    registry: ToolRegistry,
    ctx: AsyncRunnerContext,
    task: Task,
) -> RuntimeResult<RunnerResult> {
    match task.protocol_id.as_str() {
        AGENT_TOOL_LIST_PROTOCOL => service_result_event(
            PLUGIN_ID,
            &task,
            "mutsuki.agent.tool.listed",
            |request: AgentToolListRequest| Ok(registry.list(request)),
        ),
        AGENT_TOOL_EXECUTE_PROTOCOL => {
            let request: AgentToolExecuteRequest = task_payload(PLUGIN_ID, &task)?;
            let descriptor = registry
                .get(&request.name)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            if descriptor.requires_approval {
                return Err(runtime_failure(
                    PLUGIN_ID,
                    &task.task_id,
                    AgentError::new(
                        "agent.approval_required",
                        format!("tool `{}` requires approval", descriptor.name),
                    ),
                ));
            }
            let outcome = ctx
                .call_raw(descriptor.target_protocol_id.clone(), request.input.clone())
                .await?;
            let result = tool_result(request.name, outcome);
            result_event(task.task_id, "mutsuki.agent.tool.executed", result)
        }
        _ => Err(unsupported_protocol(PLUGIN_ID, &task)),
    }
}

fn tool_result(name: String, outcome: TaskOutcome) -> AgentToolExecuteResult {
    AgentToolExecuteResult {
        name,
        output: json!({ "task_outcome": outcome }),
        approved: true,
    }
}
