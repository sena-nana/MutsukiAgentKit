use mutsuki_agent_protocol::*;
use mutsuki_agent_sdk::{
    AgentMemoryActivateProtocol, AgentMemoryQueryProtocol, AgentMemoryWriteProtocol,
    orchestration_runner, result_event, runtime_failure, task_payload,
};
use mutsuki_runtime_sdk::contracts::{RunnerResult, Task};
use mutsuki_runtime_sdk::{AsyncRunnerAdapter, PluginBuilder, RuntimeClientRef, RuntimeResult};

use crate::MemoryRouter;

pub const PLUGIN_ID: &str = "mutsuki.plugin.agent.memory_router";
pub const RUNNER_ID: &str = "mutsuki.agent.memory_router.runner";

pub fn plugin(client: RuntimeClientRef, router: MemoryRouter) -> PluginBuilder {
    PluginBuilder::new(PLUGIN_ID)
        .protocol::<AgentMemoryQueryProtocol>()
        .protocol::<AgentMemoryWriteProtocol>()
        .protocol::<AgentMemoryActivateProtocol>()
        .runner(Box::new(runner(client, router)))
}

pub fn runner(client: RuntimeClientRef, router: MemoryRouter) -> AsyncRunnerAdapter {
    let descriptor = orchestration_runner(RUNNER_ID, PLUGIN_ID)
        .accepts::<AgentMemoryQueryProtocol>()
        .accepts::<AgentMemoryWriteProtocol>()
        .accepts::<AgentMemoryActivateProtocol>()
        .build();
    AsyncRunnerAdapter::new(
        descriptor,
        client,
        Box::new(move |_ctx, task| {
            let router = router.clone();
            Box::pin(async move { run_task(router, task).await })
        }),
    )
}

async fn run_task(router: MemoryRouter, task: Task) -> RuntimeResult<RunnerResult> {
    match task.protocol_id.as_str() {
        AGENT_MEMORY_QUERY_PROTOCOL => {
            let request: AgentMemoryQueryRequest = task_payload(PLUGIN_ID, &task)?;
            let result = router
                .query(request)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            result_event(task.task_id, "mutsuki.agent.memory.query_result", result)
        }
        AGENT_MEMORY_WRITE_PROTOCOL => {
            let request: AgentMemoryWriteRequest = task_payload(PLUGIN_ID, &task)?;
            let result = router
                .write(request)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            result_event(task.task_id, "mutsuki.agent.memory.written", result)
        }
        AGENT_MEMORY_ACTIVATE_PROTOCOL => {
            let request: AgentMemoryActivateRequest = task_payload(PLUGIN_ID, &task)?;
            let result = router
                .activate(request)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            result_event(task.task_id, "mutsuki.agent.memory.activated", result)
        }
        _ => Ok(RunnerResult::completed(task.task_id)),
    }
}
