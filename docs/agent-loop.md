# Agent Loop

`mutsuki.agent/run@1` 与 `mutsuki.agent.loop/step@1`。普通 orchestration runner，经
batch-first `AsyncRunnerAdapter` 接入 Core。run 通过 `AsyncRunnerContext` 生成并等待
model/tool 子 task；model 结果通过调用方声明的 result protocol callback 返回产品
业务 runner。trace/correlation/session 在子 task 上透传。approval 与长期 memory
策略仍由独立协议和 provider 管理。
