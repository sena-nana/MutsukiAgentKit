# Rust SDK

The SDK wraps Mutsuki runtime calls with typed Agent request builders and protocol marker types.

`AgentClient::run_text(...).max_steps(...).call()` submits `mutsuki.agent/run@1` through the underlying `AsyncRunnerContext`.

`#[agent_tool]` generates an Agent tool descriptor function next to the Rust function. It does not create a new runtime.

Use an SDK protocol marker for `target` when the tool target is known at compile time. String targets remain available for dynamic protocols.
