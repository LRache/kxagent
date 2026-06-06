// This example demonstrates how to register a tool with the agent core.
// use a function tool as an example.

use std::collections::HashMap;
use std::sync::Arc;

use agent_core::{AgentCoreResult, FnTool, Tool, ToolMetadata, ToolOutput, ToolSchema};

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn main() -> anyhow::Result<()> {
    // 创建一个工具注册表
    let mut tools: HashMap<String, Arc<dyn Tool>> = HashMap::new();

    // 定义一个 FnTool
    let factorial_tool = FnTool {
        metadata: ToolMetadata {
            schema: ToolSchema {
                name: "factorial".to_string(),
                description: "Calculates the factorial of a number".to_string(),
                parameters_schema: serde_json::json!({
                    "number": {"type": "number"}
                }),
            },
            default_visibility: "Direct".to_string(),
            capabilities: "math".to_string(),
            execution: "i dont know".to_string(),
        },
        handler: Box::new(|invocation| {
            let number = invocation.arguments.get("number").and_then(|v| v.as_u64()).unwrap_or(0);
            let fn_result = factorial(number);
            AgentCoreResult::Ok(ToolOutput {
                result: serde_json::json!(fn_result),
            })
        }),
    };

    // 将工具注册到工具注册表中
    tools.insert(factorial_tool.metadata().schema.name.clone(), Arc::new(factorial_tool));
    println!("成功注册了工具: {}", &tools["factorial"].metadata().schema.name);
    Ok(())
}
