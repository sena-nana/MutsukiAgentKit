use mutsuki_agent_protocol::AgentError;
use mutsuki_runtime_sdk::contracts::{
    DomainEvent, ExecutionClass, RunnerResult, RunnerStatus, ScalarValue,
};
use mutsuki_runtime_sdk::{RuntimeFailure, RuntimeResult};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub fn runtime_failure(
    source: &'static str,
    route: impl Into<String>,
    error: AgentError,
) -> RuntimeFailure {
    let mut runtime_error =
        mutsuki_runtime_sdk::contracts::RuntimeError::new(error.code, source, route.into());
    runtime_error
        .evidence
        .insert("message".into(), ScalarValue::String(error.message));
    RuntimeFailure::new(runtime_error)
}

pub fn task_payload<T>(
    source: &'static str,
    task: &mutsuki_runtime_sdk::contracts::Task,
) -> RuntimeResult<T>
where
    T: DeserializeOwned,
{
    serde_json::from_value(task.payload.clone()).map_err(|error| {
        runtime_failure(
            source,
            &task.task_id,
            AgentError::invalid_input(error.to_string()),
        )
    })
}

pub fn result_event(
    task_id: impl Into<String>,
    event_kind: impl Into<String>,
    payload: impl Serialize,
) -> RuntimeResult<RunnerResult> {
    let task_id = task_id.into();
    let payload = serde_json::to_value(payload).map_err(|error| {
        RuntimeFailure::new(mutsuki_runtime_sdk::contracts::RuntimeError::new(
            "agent.serialize_failed",
            "mutsuki.agent",
            error.to_string(),
        ))
    })?;
    let mut result = RunnerResult::completed(task_id.clone());
    result.events.push(DomainEvent {
        event_id: format!("{task_id}:result"),
        kind: event_kind.into(),
        payload,
    });
    Ok(result)
}

pub fn failed_result(task_id: impl Into<String>, error: AgentError) -> RunnerResult {
    let task_id = task_id.into();
    let mut result = RunnerResult::completed(task_id.clone());
    result.status = RunnerStatus::Failed;
    result.events.push(DomainEvent {
        event_id: format!("{task_id}:error"),
        kind: "mutsuki.agent.error".into(),
        payload: serde_json::to_value(error).unwrap_or_else(|_| serde_json::json!({})),
    });
    result
}

pub fn orchestration_runner(
    runner_id: impl Into<String>,
    plugin_id: impl Into<String>,
) -> mutsuki_runtime_sdk::RunnerDescriptorBuilder {
    mutsuki_runtime_sdk::RunnerDescriptorBuilder::new(runner_id, plugin_id)
        .execution_class(ExecutionClass::Orchestration)
}
