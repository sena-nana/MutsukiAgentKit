# Agent TestKit Skill

Use this when adding conformance helpers, fake providers, fake tools, or examples.

## Rules

- Tests must assert behavior through public service, SDK, or runner surfaces.
- Do not add tests that only match logs, fixed prose, or implementation-specific strings unless the string is itself a public protocol value.
- Prefer small round-trip checks: create/get/append, write/query, register/render, run/complete.
- Fake providers belong in `crates/mutsuki-agent-testkit`; production providers belong in plugin crates or separate model plugin repositories.

## Checklist

1. Add a helper that exercises the public API the same way a consumer would.
2. Keep fixtures minimal and typed.
3. Run targeted package tests first, then workspace tests.
