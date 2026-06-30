# Tool Router

`mutsuki-plugin-agent-tool-router` stores Agent tool descriptors and dispatches `mutsuki.agent.tool/execute@1` to the descriptor target protocol.

Current runtime SDK task outcomes do not expose typed child output values, so the router result contains the child task outcome in its domain event payload.
