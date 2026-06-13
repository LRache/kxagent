// 此示例演示如何从 agent-core 库中获取工具并注册到工具注册表中。

use std::collections::HashMap;
use std::sync::Arc;

use agent_core::{Tool, FnTool,ToolMetadata,ToolSchema,AgentCoreResult,ToolOutput};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建工具注册表
    let mut tools: HashMap<String, Arc<dyn Tool>> = HashMap::new();

    // 手动创建一个Fntool
    let add_tool =  FnTool {
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
            let left = invocation
                .arguments
                .get("left")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let right = invocation
                .arguments
                .get("right")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            AgentCoreResult::Ok(ToolOutput {
                result: serde_json::json!(left + right),
            })
        }),
    };
  
    // 注册
    let name = add_tool.metadata().schema.name.clone();
    tools.insert(name.clone(), Arc::new(add_tool));
    println!("成功注册了工具: {name}");


    // 验证：从注册表取出并调用
    let inv = agent_core::ToolInvocation {
        call_id: "demo-1".to_string(),
        tool_name: "add".to_string(),
        arguments: serde_json::json!({"left": 3, "right": 7}),
        metadata: tools["add"].metadata().clone(),
    };
    
    let output = tools["add"].invoke(inv).await?;
    println!("add(3, 7) = {}", output.result);

    Ok(())
}
