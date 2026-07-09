# MutsukiAgentKit

MutsukiAgentKit 是面向 MutsukiCore-style runtime 的 Rust-native Agent 协议 / SDK / 插件集合，不是 Core、ServiceHost 或 TauriHost 的扩展补丁。

本仓库不含 Python Agent SDK。

## 分层边界

```text
MutsukiCore       TaskPool / scheduler / ResultRouter
ServiceHost       process supervision / plugin discovery
TauriHost         UI / desktop shell
StdPlugins        fs / http / sqlite / config / observe
BotPlugins        bot platform adapters
MutsukiAgentKit   Agent protocols + SDK + workflow plugins
```

## 承担 / 不拥有

**承担：** Agent protocols、authoring helpers、workflow plugins、LLM/tool adapters、memory/resource helpers、conformance。

**不拥有：** Core scheduler、HostRuntime、Tauri/Service daemon、Python runner bridge、Bot adapters、StdPlugins baseline。不声明 Core 内建 AgentLoop / TaskGroup / actor。

## 红线

1. 不复制 scheduler；async 只经 `RuntimeClient` / `TaskHandle` / `TaskAwait`
2. memory / stream 用 `ResourceRef` / `ResourceCellRef`，不写 Core StateStore 私有路径
3. 外部副作用走 effectful runner / 标准 provider；tool call 经 TaskPool
4. 不依赖 ServiceHost / TauriHost 私有 API

## Workspace

- `crates/mutsuki-agent-protocol` · `mutsuki-agent-sdk` · `mutsuki-agent-macros`
- `crates/mutsuki-plugin-agent-*` · `mutsuki-agent-testkit`
- `schemas/` · `manifests/` · `examples/` · `templates/` · `docs/`

## 协议映射（Issue #1）

| 草案 | 稳定 id |
| --- | --- |
| plan | `mutsuki.agent/run@1` |
| step | `mutsuki.agent.loop/step@1` |
| tool.call | `mutsuki.agent.tool/execute@1` |
| memory.query / write | `mutsuki.agent.memory/query@1` · `.../write@1` |
| llm.complete / stream | `mutsuki.agent.model/generate@1` · `.../stream@1` |

详见 [`docs/architecture.md`](docs/architecture.md)、[`docs/protocol.md`](docs/protocol.md)。
