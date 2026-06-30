# Architecture

MutsukiAgentKit is a Rust-native Agent kit. It is made of protocol DTOs, Rust SDK wrappers, proc macros, native plugin crates, manifests, schemas, examples, and a testkit.

The runtime boundary is protocol-first:

```text
MutsukiCore
  -> Rust native plugin runner
  -> MutsukiAgentKit protocol
  -> service implementation
  -> domain event result
```

Plugin service APIs are intentionally kept separate from runner glue so behavior can be tested without a host runtime.
