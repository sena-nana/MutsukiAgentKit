use mutsuki_agent_protocol::*;
use mutsuki_agent_sdk::{
    AgentSessionAppendProtocol, AgentSessionCreateProtocol, AgentSessionGetProtocol,
    AgentSessionSnapshotProtocol, orchestration_runner, result_event, runtime_failure,
    task_payload,
};
use mutsuki_runtime_sdk::contracts::{RunnerResult, Task};
use mutsuki_runtime_sdk::{AsyncRunnerAdapter, PluginBuilder, RuntimeClientRef, RuntimeResult};

use crate::SessionStore;

pub const PLUGIN_ID: &str = "mutsuki.plugin.agent.session";
pub const RUNNER_ID: &str = "mutsuki.agent.session.runner";

pub fn plugin(client: RuntimeClientRef, store: SessionStore) -> PluginBuilder {
    PluginBuilder::new(PLUGIN_ID)
        .protocol::<AgentSessionCreateProtocol>()
        .protocol::<AgentSessionGetProtocol>()
        .protocol::<AgentSessionAppendProtocol>()
        .protocol::<AgentSessionSnapshotProtocol>()
        .runner(Box::new(runner(client, store)))
}

pub fn runner(client: RuntimeClientRef, store: SessionStore) -> AsyncRunnerAdapter {
    let descriptor = orchestration_runner(RUNNER_ID, PLUGIN_ID)
        .accepts::<AgentSessionCreateProtocol>()
        .accepts::<AgentSessionGetProtocol>()
        .accepts::<AgentSessionAppendProtocol>()
        .accepts::<AgentSessionSnapshotProtocol>()
        .build();
    AsyncRunnerAdapter::new(
        descriptor,
        client,
        Box::new(move |_ctx, task| {
            let store = store.clone();
            Box::pin(async move { run_task(store, task).await })
        }),
    )
}

async fn run_task(store: SessionStore, task: Task) -> RuntimeResult<RunnerResult> {
    match task.protocol_id.as_str() {
        AGENT_SESSION_CREATE_PROTOCOL => {
            let request: AgentSessionCreateRequest = task_payload(PLUGIN_ID, &task)?;
            result_event(
                task.task_id,
                "mutsuki.agent.session.created",
                store
                    .create(request)
                    .map_err(|error| runtime_failure(PLUGIN_ID, "agent.session.create", error))?,
            )
        }
        AGENT_SESSION_GET_PROTOCOL => {
            let request: AgentSessionGetRequest = task_payload(PLUGIN_ID, &task)?;
            result_event(
                task.task_id,
                "mutsuki.agent.session.loaded",
                store
                    .get(request)
                    .map_err(|error| runtime_failure(PLUGIN_ID, "agent.session.get", error))?,
            )
        }
        AGENT_SESSION_APPEND_PROTOCOL => {
            let request: AgentSessionAppendRequest = task_payload(PLUGIN_ID, &task)?;
            result_event(
                task.task_id,
                "mutsuki.agent.session.appended",
                store
                    .append(request)
                    .map_err(|error| runtime_failure(PLUGIN_ID, "agent.session.append", error))?,
            )
        }
        AGENT_SESSION_SNAPSHOT_PROTOCOL => {
            let request: AgentSessionSnapshotRequest = task_payload(PLUGIN_ID, &task)?;
            result_event(
                task.task_id,
                "mutsuki.agent.session.snapshot",
                store
                    .snapshot(request)
                    .map_err(|error| runtime_failure(PLUGIN_ID, "agent.session.snapshot", error))?,
            )
        }
        _ => Ok(RunnerResult::completed(task.task_id)),
    }
}
