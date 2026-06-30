# Repository Instructions

MutsukiAgentKit is a Rust-native Agent plugin and SDK collection for MutsukiCore-style runtimes.

## Boundaries

- Do not add Python SDK packages, Python decorators, Python runners, or Python-first Agent runtime code to this repository.
- Keep protocol DTOs in `crates/mutsuki-agent-protocol`; plugin crates must not define competing wire shapes.
- Keep Rust SDK ergonomics in `crates/mutsuki-agent-sdk`; plugin crates should expose services and plugin builders, not ad hoc client wrappers.
- Keep proc macros in `crates/mutsuki-agent-macros`; macros may attach metadata or generate descriptors, but they must not become a separate tool runtime.
- Runtime-facing plugin paths should use Mutsuki protocols rather than direct plugin-to-plugin calls. Pure service APIs are allowed for tests, embedding, and deterministic logic.

## Implementation Rules

- Prefer small, typed structs and explicit protocol ids over stringly typed local conventions.
- Do not add placeholder UI, mock-looking public features, or documentation that claims runtime behavior without an implemented Rust API.
- Tests must exercise behavior through services, builders, protocol descriptors, or runner outputs. Avoid tests that only match log text or fixed prose.
- Before adding a dependency, check whether the protocol crate, SDK crate, or existing runtime SDK already provides the needed shape.
- When adding a new protocol, update protocol constants, SDK marker types, manifests, schemas, and at least one concrete service or runner path together.

## Development Directions

- Protocol work: start with `skills/protocol/SKILL.md`.
- SDK and macro work: start with `skills/sdk/SKILL.md`.
- Plugin runtime work: start with `skills/plugins/SKILL.md`.
- Conformance and examples: start with `skills/testkit/SKILL.md`.
