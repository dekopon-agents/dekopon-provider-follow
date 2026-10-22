# Guidance for coding agents

This repository builds one conversation follow lifecycle Wasm component. This file and its linked documents are the whole contract, including for an agent in a bare checkout. Read [README.md](README.md) for its interface and limitations. Core's [boundaries](https://github.com/dekopon-agents/dekopon/blob/main/AGENTS.md#boundaries-that-must-survive), [Rust guidelines](https://github.com/dekopon-agents/dekopon/blob/main/AGENTS.md#rust-guidelines), and [comments](https://github.com/dekopon-agents/dekopon/blob/main/AGENTS.md#comments) apply. No releases, grants, bindings, live calls, or version bumps without owner approval.

## Boundaries

- Capability IDs are exactly `dekopon-provider-follow.stay` and `dekopon-provider-follow.leave`.
- Both inputs are closed empty objects, with `Needs = ()`, `LocalWrite` and `Low`.
- Runs write nothing and succeed; the gateway owns tickets, debounce, lapse and self-leave.
- Only the broker grants authority. Follow does not grant other capabilities or enable recall.

## Build and verify

The shared [provider-workflows](https://github.com/dekopon-agents/provider-workflows) job is `ci / validate`. Clone it as a sibling in a bare checkout with `git clone --depth 1 https://github.com/dekopon-agents/provider-workflows ../provider-workflows`. Before pushing run:

```console
cargo fmt --all --check
cargo deny --all-features check bans licenses sources advisories
cargo clippy -p dekopon-follow-provider --all-targets --locked -- -D warnings
cargo clippy -p dekopon-follow-provider --lib --target wasm32-unknown-unknown --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc -p dekopon-follow-provider --no-deps --locked
../provider-workflows/build.sh
DEKOPON_PROVIDER_COMPONENT="$PWD/follow-provider.wasm" cargo test -p dekopon-follow-provider --locked
```

`build.sh` uses this checkout's target. Built-component tests must read `DEKOPON_PROVIDER_COMPONENT` with `expect`, never skip or silently fall back. Use the published testkit's `Harness` and conformance checks. Name native-only tests as such. Match typed errors rather than Display. Use synthetic credentials only. Pin SDK and testkit to the same published exact version, regenerate lock with Cargo. No `CHANGELOG.md` change in a feature PR; commit has one physical `Changelog:` trailer per user-facing entry. PRs never push main.
