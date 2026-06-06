# kxagent
An agent core in Rust

现在的目录结构：
  
```
kxagent
├─ Cargo.toml
├─ crates
│  ├─ agent-cli
│  │  ├─ Cargo.toml
│  │  └─ src
│  │     └─ main.rs
│  └─ agent-core
│     ├─ Cargo.toml
│     └─ src
│        ├─ app
│        ├─ component
│        │  └─ tool
│        │     └─ mod.rs
│        ├─ graph
│        ├─ lib.rs
│        ├─ report
│        ├─ runtime
│        └─ types
│           └─ agenticMessage.rs
├─ examples
│  └─ register_tool.rs
├─ LICENSE
├─ README.md
├─ rustfmt.toml
└─ src
   ├─ lib.rs
   ├─ main.rs
   └─ types.rs

```

```
预期的目录结构如下：
agent-core/
├── Cargo.toml
├── crates/
│   ├── agent-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── 
│   │       │
│   │       ├── graph/
│   │       │   ├── mod.rs
│   │       │   ├── node.rs
│   │       │   ├── edge.rs
│   │       │   ├── state.rs
│   │       │   └── builder.rs
│   │       │
│   │       ├── runtime/
│   │       │   ├── mod.rs
│   │       │   ├── executor.rs
│   │       │   ├── run_turn.rs
│   │       │   ├── context.rs
│   │       │   ├── event.rs
│   │       │   └── error.rs
│   │       │
│   │       ├── component/
│   │       │   ├── mod.rs
│   │       │   ├── tool/
│   │       │   │   ├── mod.rs
│   │       │   │   ├── registry.rs
│   │       │   │   ├── spec.rs
│   │       │   │   └── result.rs
│   │       │   ├── memory/
│   │       │   │   └── mod.rs
│   │       │   └── model/
│   │       │       └── mod.rs
│   │       │
│   │       ├── app/
│   │       │   ├── mod.rs
│   │       │   ├── phoneuse.rs
│   │       │   └── mock.rs
│   │       │
│   │       └── report/
│   │           ├── mod.rs
│   │           └── trace.rs
│   │
│   └── agent-cli/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
├── examples/
│   ├── minimal_graph.rs
│   └── tool_call.rs
│
├── tests/
│   └── graph_runtime.rs
│
└── docs/
    ├── architecture.md
    └── runtime.md
```