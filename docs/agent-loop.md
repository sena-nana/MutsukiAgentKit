# Agent Loop

`mutsuki.agent/run@1` 与 `mutsuki.agent.loop/step@1`。普通 orchestration runner，经 `run_batch` 接入 Core。MVP 为确定性单步生成；多步 tool / approval / 长期 memory 未声明完成。
