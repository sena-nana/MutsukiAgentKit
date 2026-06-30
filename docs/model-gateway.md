# Model Gateway

`mutsuki-plugin-agent-model-gateway` routes generate requests to registered Rust `ModelProvider` implementations.

The default provider is a mock provider for local conformance and examples. External providers should be Rust plugins or separate model plugin crates.
