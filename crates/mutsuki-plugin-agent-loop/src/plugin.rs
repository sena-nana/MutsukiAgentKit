use mutsuki_agent_protocol::*;
use mutsuki_agent_sdk::{
    AgentLoopStepProtocol, AgentRunProtocol, orchestration_runner, service_result_event,
    unsupported_protocol,
};
use mutsuki_runtime_sdk::AsyncRunnerContext;
use mutsuki_runtime_sdk::contracts::{RunnerResult, Task};
use mutsuki_runtime_sdk::{AsyncRunnerAdapter, PluginBuilder, RuntimeClientRef, RuntimeResult};

use crate::AgentLoop;

pub const PLUGIN_ID: &str = "mutsuki.plugin.agent.loop";
pub const RUNNER_ID: &str = "mutsuki.agent.loop.runner";

pub fn plugin(client: RuntimeClientRef, agent_loop: AgentLoop) -> PluginBuilder {
    PluginBuilder::new(PLUGIN_ID)
        .protocol::<AgentRunProtocol>()
        .protocol::<AgentLoopStepProtocol>()
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
        Box::new(move |ctx, task| {
            let agent_loop = agent_loop.clone();
            Box::pin(async move { run_task(agent_loop, ctx, task).await })
        }),
    )
}

async fn run_task(
    agent_loop: AgentLoop,
    ctx: AsyncRunnerContext,
    task: Task,
) -> RuntimeResult<RunnerResult> {
    match task.protocol_id.as_str() {
        AGENT_RUN_PROTOCOL => {
            let request: AgentRunRequest = mutsuki_agent_sdk::task_payload(PLUGIN_ID, &task)?;
            let planned = agent_loop.plan_tasks(&request, &task).map_err(|error| {
                mutsuki_agent_sdk::runtime_failure(PLUGIN_ID, &task.task_id, error)
            })?;
            for child in planned
                .into_iter()
                .filter(|child| child.protocol_id != AGENT_LOOP_STEP_PROTOCOL)
            {
                let outcome = ctx.call_raw(child.protocol_id, child.payload).await?;
                if !matches!(
                    outcome,
                    mutsuki_runtime_sdk::contracts::TaskOutcome::Completed { .. }
                ) {
                    return Err(mutsuki_agent_sdk::runtime_failure(
                        PLUGIN_ID,
                        &task.task_id,
                        AgentError::new(
                            "agent.child_failed",
                            format!("Agent child task did not complete: {outcome:?}"),
                        ),
                    ));
                }
            }
            service_result_event(
                PLUGIN_ID,
                &task,
                "mutsuki.agent.run.completed",
                |request: AgentRunRequest| agent_loop.run(request),
            )
        }
        AGENT_LOOP_STEP_PROTOCOL => service_result_event(
            PLUGIN_ID,
            &task,
            "mutsuki.agent.loop.step_completed",
            |request: AgentLoopStepRequest| agent_loop.step(request),
        ),
        _ => Err(unsupported_protocol(PLUGIN_ID, &task)),
    }
}
