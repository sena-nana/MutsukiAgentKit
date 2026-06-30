# Protocol

Protocol ids are defined in `crates/mutsuki-agent-protocol/src/lib.rs`.

The MVP surface is:

- `mutsuki.agent/run@1`
- `mutsuki.agent.loop/step@1`
- `mutsuki.agent.context/build@1`
- `mutsuki.agent.tool/list@1`
- `mutsuki.agent.tool/execute@1`
- `mutsuki.agent.session/create@1`
- `mutsuki.agent.session/get@1`
- `mutsuki.agent.session/append@1`
- `mutsuki.agent.session/snapshot@1`
- `mutsuki.agent.memory/query@1`
- `mutsuki.agent.memory/write@1`
- `mutsuki.agent.memory/activate@1`
- `mutsuki.agent.model/generate@1`
- `mutsuki.agent.prompt/render@1`
- `mutsuki.agent.prompt/get@1`
