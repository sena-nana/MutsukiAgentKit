# Agent Protocol Skill

Use this when changing Agent protocol DTOs, protocol ids, JSON schemas, or manifest protocol lists.

## Rules

- Keep all wire DTOs in `crates/mutsuki-agent-protocol`.
- Add or rename protocol ids only in `crates/mutsuki-agent-protocol/src/lib.rs`.
- Mirror every protocol id with a marker type in `crates/mutsuki-agent-sdk/src/protocol.rs`.
- Update `schemas/` and `manifests/` in the same change when payloads or protocol surfaces change.
- Do not introduce Python protocol wrappers in this repository.

## Checklist

1. Define the typed request/result structs in the protocol crate.
2. Export them from `crates/mutsuki-agent-protocol/src/lib.rs`.
3. Add or update SDK protocol marker types.
4. Update manifests for providers and consumers.
5. Add behavior-level tests only when the protocol change affects runtime behavior.
