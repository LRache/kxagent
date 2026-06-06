//! KxAgent CLI 入口文件。

use agent_core::{AgentCoreResult, FnTool, Tool, ToolMetadata, ToolOutput, ToolSchema, add};
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tools: HashMap<String, Arc<dyn Tool>> = HashMap::new();

    // 定义一个 FnTool 做加法
    let add_tool = FnTool {
        metadata: ToolMetadata {
            schema: ToolSchema {
                name: "add".to_string(),
                description: "Adds two numbers".to_string(),
                parameters_schema: serde_json::json!({
                    "left":  {"type": "number"},
                    "right": {"type": "number"}
                }),
            },
            default_visibility: "Direct".to_string(),
            capabilities: "math".to_string(),
            execution: "sync".to_string(),
        },
        handler: Box::new(|invocation| {
            let left = invocation.arguments.get("left").and_then(|v| v.as_u64()).unwrap_or(0);
            let right = invocation.arguments.get("right").and_then(|v| v.as_u64()).unwrap_or(0);
            let fn_result = add(left, right);
            AgentCoreResult::Ok(ToolOutput {
                result: serde_json::json!(fn_result),
            })
        }),
    };

    tools.insert(add_tool.metadata().schema.name.clone(), Arc::new(add_tool));

    println!("Registered {} tool(s)", tools.len());
    Ok(())
}
