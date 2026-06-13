//! 内置工具构造函数。

use crate::types::{AgentCoreResult, FnTool, ToolMetadata, ToolOutput, ToolSchema};

/// 创建一个 factorial 工具：计算 n!。
pub fn factorial_tool() -> FnTool {
    FnTool {
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
            execution: "sync".to_string(),
        },
        handler: Box::new(|invocation| {
            let n = invocation
                .arguments
                .get("number")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let result: u64 = (1..=n).product();
            AgentCoreResult::Ok(ToolOutput {
                result: serde_json::json!(result),
            })
        }),
    }
}

/// 创建一个 add 工具：两数相加。
pub fn add_tool() -> FnTool {
    FnTool {
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
    }
}
