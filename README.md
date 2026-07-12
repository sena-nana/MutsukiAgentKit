# MutsukiAgentKit

MutsukiAgentKit 是面向 MutsukiCore-style runtime 的 Rust-native Agent 协议 / SDK / 插件集合，不是 Core 或任何产品 Host 的扩展补丁。

本仓库不含 Python Agent SDK。

## 分层边界

```text
MutsukiCore       TaskPool / scheduler / ResultRouter
Product Host      process supervision / plugin discovery / configuration
Platform adapters product-specific ingress and egress
MutsukiAgentKit   Agent protocols + SDK + workflow plugins
```

## 承担 / 不拥有

**承担：** Agent protocols、authoring helpers、workflow plugins、LLM/tool adapters、memory/resource helpers、conformance。

**不拥有：** Core scheduler、HostRuntime、产品配置与 Secret 解析、daemon、Python runner bridge、平台 adapters、StdPlugins baseline。不声明 Core 内建 AgentLoop / TaskGroup / actor。

## 红线

1. 不复制 scheduler；async 只经 `RuntimeClient` / `TaskHandle` / `TaskAwait`
2. memory / stream 用 `ResourceRef` / `ResourceCellRef`，不写 Core StateStore 私有路径
3. 外部副作用走 effectful runner / 标准 provider；tool call 经 TaskPool
4. 不依赖产品 Host，不读取配置文件或环境变量，不解析或保存产品 Secret

## Workspace

- `crates/mutsuki-agent-protocol` · `mutsuki-agent-sdk` · `mutsuki-agent-macros`
- `crates/mutsuki-plugin-agent-*` · `mutsuki-agent-testkit`
- `crates/mutsuki-agent-bundle` · Host-neutral Agent services 与 manifest 集合
- `schemas/` · `manifests/` · `examples/` · `templates/` · `docs/`

## 协议映射（Issue #1）

| 草案 | 稳定 id |
| --- | --- |
| plan | `mutsuki.agent/run@1` |
| step | `mutsuki.agent.loop/step@1` |
| tool.call | `mutsuki.agent.tool/execute@1` |
| memory.query / write | `mutsuki.agent.memory/query@1` · `.../write@1` |
| llm.complete / stream | `mutsuki.agent.model/generate@1` · `.../stream@1` |

Model gateway 是 provider-neutral orchestration runner。真实 HTTP provider 经
`effect.mutsuki.agent.model/http@1` effect runner 执行；消费端显式构造 provider 并注入
运行参数与 credential。AgentKit 不读取或管理配置，credential 不进入 task、trace 或普通日志。stream 正文保存在
provider 资源存储，runtime task 只携带 `ResourceRef`。

详见 [`docs/architecture.md`](docs/architecture.md)、[`docs/protocol.md`](docs/protocol.md)。
