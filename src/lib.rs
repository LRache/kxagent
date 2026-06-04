//! TODO: Add a description of the library here.
//! This an agent core library, which provides the basic functionalities for the agent.

#![deny(missing_docs)]

/// Agent 核心类型（Tool trait、Schema、错误类型等）。
pub mod types;

pub use types::{AgentCoreResult, AgentError, FnTool, Tool, ToolInvocation, ToolMetadata, ToolOutput, ToolSchema};

/// A simple library to test the build process of a Rust project.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    /// 测试 add_tool：不依赖 LLM，直接构造 ToolInvocation 调用 invoke。
    #[tokio::test]
    async fn test_add_tool() {
        // 1. 构造 FnTool
        let metadata = ToolMetadata {
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
        };

        let add_tool = FnTool {
            metadata: metadata.clone(),
            handler: Box::new(|invocation| {
                let left = invocation.arguments.get("left").and_then(|v| v.as_u64()).unwrap_or(0);
                let right = invocation.arguments.get("right").and_then(|v| v.as_u64()).unwrap_or(0);
                let fn_result = add(left, right);
                return AgentCoreResult::Ok(ToolOutput {
                    result: serde_json::json!(fn_result),
                });
            }),
        };

        // 2. 构造一次工具调用
        let invocation = ToolInvocation {
            call_id: "test-1".to_string(),
            tool_name: "add".to_string(),
            arguments: serde_json::json!({"left": 3, "right": 5}),
            metadata,
        };

        // 3. 执行
        let result: AgentCoreResult<ToolOutput> = add_tool.invoke(invocation).await;

        // 4. 验证结果
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
        assert_eq!(result.unwrap().result, serde_json::json!(8));
    }
}
