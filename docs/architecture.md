# Architecture

MutsukiAgentKit 是 Core 之上的高层 Agent 能力库，不是 Host 补丁。

```text
Host / Bot adapters
  -> AgentKit (protocols · SDK · plugins)
  -> StdPlugins
  -> MutsukiCore (batch-first TaskPool)
```

Runtime 路径：`WorkBatch` → `run_batch` → `CompletionBatch`（经 `AsyncRunnerAdapter`）。descriptor 声明 batch / payload / resources / ordering / control。

## 红线

- 不复制 scheduler；不接管语言 event loop
- memory / stream / LLM output 用 `ResourceRef` / `ResourceCellRef`
- LLM / 工具副作用走 effectful runner 或标准 plugin，不本地直调绕过 TaskPool
- 只依赖 `mutsuki-runtime-sdk`，不依赖 ServiceHost / TauriHost 私有 API
- 不声明 Core 内建 Agent 能力

## 非目标

不实现 Core workflow / actor / TaskGroup；不把商业 LLM 集成声明为 Core 功能。
