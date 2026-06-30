use mutsuki_agent_protocol::*;
use mutsuki_agent_sdk::{
    AgentLoopStepProtocol, AgentRunProtocol, orchestration_runner, result_event, runtime_failure,
    task_payload,
};
use mutsuki_runtime_sdk::contracts::{RunnerResult, Task};
use mutsuki_runtime_sdk::{AsyncRunnerAdapter, PluginBuilder, RuntimeClientRef, RuntimeResult};

use crate::AgentLoop;

pub const PLUGIN_ID: &str = "mutsuki.plugin.agent.loop";
pub const RUNNER_ID: &str = "mutsuki.agent.loop.runner";

pub fn plugin(client: RuntimeClientRef, agent_loop: AgentLoop) -> PluginBuilder {
    PluginBuilder::new(PLUGIN_ID)
        .protocol::<AgentRunProtocol>()
        .protocol::<AgentLoopStepProtocol>()
        .requires(AGENT_CONTEXT_BUILD_PROTOCOL)
        .requires(AGENT_MODEL_GENERATE_PROTOCOL)
        .requires(AGENT_TOOL_EXECUTE_PROTOCOL)
        .requires(AGENT_SESSION_APPEND_PROTOCOL)
        .runner(Box::new(runner(client, agent_loop)))
}

pub fn runner(client: RuntimeClientRef, agent_loop: AgentLoop) -> AsyncRunnerAdapter {
    let descriptor = orchestration_runner(RUNNER_ID, PLUGIN_ID)
        .accepts::<AgentRunProtocol>()
        .accepts::<AgentLoopStepProtocol>()
        .build();
    AsyncRunnerAdapter::new(
        descriptor,
        client,
        Box::new(move |_ctx, task| {
            let agent_loop = agent_loop.clone();
            Box::pin(async move { run_task(agent_loop, task).await })
        }),
    )
}

async fn run_task(agent_loop: AgentLoop, task: Task) -> RuntimeResult<RunnerResult> {
    match task.protocol_id.as_str() {
        AGENT_RUN_PROTOCOL => {
            let request: AgentRunRequest = task_payload(PLUGIN_ID, &task)?;
            let result = agent_loop
                .run(request)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            result_event(task.task_id, "mutsuki.agent.run.completed", result)
        }
        AGENT_LOOP_STEP_PROTOCOL => {
            let request: AgentLoopStepRequest = task_payload(PLUGIN_ID, &task)?;
            let result = agent_loop
                .step(request)
                .map_err(|error| runtime_failure(PLUGIN_ID, &task.task_id, error))?;
            result_event(task.task_id, "mutsuki.agent.loop.step_completed", result)
        }
        _ => Ok(RunnerResult::completed(task.task_id)),
    }
}
