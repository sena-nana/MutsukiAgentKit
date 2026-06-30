# MutsukiAgentKit

MutsukiAgentKit is the Rust-native Agent plugin and SDK kit for MutsukiCore-style runtimes. It contains Agent protocol types, Rust SDK helpers, proc macros, builtin/native plugin crates, manifests, schemas, examples, templates, and test utilities.

This repository intentionally does not contain a Python Agent SDK. Python tools can still be called through MutsukiCore protocols, but AgentKit itself is Rust protocol, Rust SDK, Rust macros, Rust plugins, and Rust test support.

## Workspace

- `crates/mutsuki-agent-protocol`: stable Agent domain DTOs and protocol id constants.
- `crates/mutsuki-agent-sdk`: Rust client/builders plus runtime protocol marker types.
- `crates/mutsuki-agent-macros`: Agent metadata macros for Rust tools and profiles.
- `crates/mutsuki-plugin-agent-*`: builtin/native Agent plugins.
- `crates/mutsuki-agent-testkit`: fake model, tool, memory, session, and conformance helpers.
- `schemas/`: JSON schema contracts for protocol payloads.
- `manifests/`: standard plugin manifest descriptors.
- `examples/`: minimal Rust Agent, tool, and provider examples.
- `templates/`: starter templates for external Agent extensions.

## Core Boundary

Agent plugins communicate through protocols such as `mutsuki.agent/run@1`, `mutsuki.agent.context/build@1`, `mutsuki.agent.model/generate@1`, and `mutsuki.agent.tool/execute@1`. Direct function calls between plugins are kept inside local service tests and embedding helpers; runtime orchestration remains protocol-first so builtin and ABI deployments share the same contract shape.
