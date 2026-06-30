# Agent SDK And Macro Skill

Use this when changing Rust SDK ergonomics, Agent clients, builders, or proc macros.

## Rules

- SDK helpers must wrap Mutsuki runtime protocols; they must not create a second Agent runtime.
- `#[agent_tool]` attaches Agent metadata to a Rust protocol handler. It does not replace Mutsuki task dispatch.
- Keep macro output small and inspectable: descriptor functions, profile constants, or compile-time validation.
- Prefer typed builders over free-form JSON when the protocol crate has a struct.

## Checklist

1. Check whether `mutsuki-runtime-sdk` already exposes the runtime primitive.
2. Add SDK wrappers in `crates/mutsuki-agent-sdk/src/`.
3. Add macro expansion in `crates/mutsuki-agent-macros/src/lib.rs` only if repeated handwritten descriptors would be error-prone.
4. Verify `cargo test -p mutsuki-agent-sdk -p mutsuki-agent-macros` when possible.
