# Agent Plugin Skill

Use this when adding or changing Rust native Agent plugins.

## Rules

- Every plugin crate must expose a pure service API and a runtime `plugin(client, service)` or `runner(client, service)` path.
- Runtime entrypoints must register `ProtocolSpec` marker types and stable runner descriptors.
- Plugin-to-plugin behavior should prefer Mutsuki protocols. Direct service composition is acceptable for local deterministic behavior and tests, but do not hide it as runtime routing.
- Output from runner tasks should be serialized into domain events until the runtime SDK exposes typed output value reads.
- Do not add placeholder integrations. A provider/router/session path must actually store, route, render, or generate something.

## Checklist

1. Put deterministic behavior in a service module.
2. Parse task payloads into protocol crate DTOs.
3. Convert service errors into runtime failures with Agent error codes.
4. Emit a domain event containing the typed result.
5. Update the plugin manifest and any relevant conformance helper.
