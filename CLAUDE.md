# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
# Build the entire workspace
cargo build

# Build a specific crate
cargo build -p agent-core

# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p agent-core

# Run a single test
cargo test -p agent-core -- test_add_tool

# Check formatting (CI also runs this)
cargo fmt --check

# Auto-format
cargo fmt

# Run an example
cargo run --example register_tool
```

## Architecture

This is a **Cargo workspace** (`resolver = "3"`, `edition = "2024"`) with two member crates under `crates/`:

- **`agent-core`** — The agent framework library. Defines the core abstractions: `Tool` trait, tool metadata/schema, invocation/result types, and error types. Currently the only implemented module is `types/`; other modules (`graph/`, `runtime/`, `component/`, `app/`, `report/`) are planned but not yet built.
- **`agent-cli`** — The binary entry point. Currently a placeholder.

Dependency versions are managed centrally in the root `[workspace.dependencies]`; sub-crates reference them via `.workspace = true`.

## Key Design Decisions

- **`Tool` trait uses `async_trait`** — all tool invocations are async, even if the handler is synchronous (`FnTool` wraps a sync closure and satisfies the async trait).
- **`AgentCoreResult<T>` is `Result<T, AgentError>`** — the universal result type for all agent operations.
- **`FnTool`** is the simplest tool implementation: pair a `ToolMetadata` with a boxed closure. For more complex tools, implement `Tool` directly.
- **`ToolSchema.parameters_schema`** is a raw `serde_json::Value` — no strong typing for tool parameters at the schema level (parameters are validated at runtime from `ToolInvocation.arguments`).

## Old Code Cleanup

The root `src/` directory has been removed. The old root-level crate code (`src/lib.rs`, `src/main.rs`, `src/types.rs`) was migrated into `crates/agent-core/` and `crates/agent-cli/`. The `examples/` directory still references the old crate name `kxagent` and needs updating to reference `agent-core` instead.
