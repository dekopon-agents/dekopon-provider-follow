# dekopon-provider-follow

Follow and banish lifecycle capabilities for Dekopon chat, as a Wasm component.
Keep reading this conversation without mentions using Dekopon core's Discord follow mode.

## Commands

```console
follow stay   # Keep reading this channel/thread for 24 hours, renewed by each reply.
follow leave  # Stop reading unmentioned messages here.
```

Both commands take no arguments. Their capability IDs are exactly
`dekopon-provider-follow.stay` and `dekopon-provider-follow.leave`, with closed,
empty object inputs (`{}`), `LocalWrite` effects and `Low` risk. Successful calls
produce no output and need no host effects (`Needs = ()`); the component imports
only the SDK's standard I/O interface. Core v0.50.0's gateway applies the
transition after the broker-authorized capability succeeds; the provider stores
nothing. Tickets, debounce, the 24-hour lapse and self-leave belong to the gateway.
Follow does not enable recall or durable memory.

## Grant and route

[examples/follow.cedar](examples/follow.cedar) grants both actions to principal
`alice` only for agent `assistant` through `dekopon-gatewayd`. Adapt those names to
your broker directory. The agent also needs its existing `agent.prompt` grant,
the provider and capabilities registered in broker configuration, and an enabled
Discord channel/thread route with `follow: true`. A proposal alone grants nothing.

## Build and verify

Rust 1.98.1 and SDK/testkit =0.50.0 are pinned. With
[provider-workflows](https://github.com/dekopon-agents/provider-workflows) checked
out beside this repository, run the gates in [AGENTS.md](AGENTS.md).
`../provider-workflows/build.sh` produces `follow-provider.wasm` and its checksum.

## Release

Version 0.1.0 is prepared for the owner's release after merge. The owner runs the
`cut-release` workflow; the tag-triggered shared workflow publishes the component.
No release is created by opening this PR.

Licensed under MIT OR Apache-2.0.
