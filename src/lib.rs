//! TODO: Add a description of the library here.
//! This an agent core libary, which provides the basic functionalities for the agent.

#![deny(missing_docs)]

use anyhow;
use async_trait::async_trait;
use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::fmt::{Arguments, Debug};
use std::{collections::HashMap, env, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex; //用来给trait处理async函数返回的Future类型

#[async_trait]
trait Tool: Debug + Send + Sync {
    // 工具元信息。
    fn metadata(&self) -> &ToolMetadata;

    // 执行一次工具调用。
    async fn invoke(&self, invocation: ToolInvocation) -> AgentCoreResult<ToolOutput>;
}

#[derive(Debug, Clone)]
struct ToolMetadata {
    schema: ToolSchema,
    default_visibility: String, // Agent 未显式配置时使用的默认可见性。
    capabilities: String,       // 工具能力标记。
    execution: String,          // 超时、可中断性和结果返回策略。
}

#[derive(Debug, Clone)]
struct ToolSchema {
    // 工具名称。
    name: String,
    // 工具描述。
    description: String,
    // 工具参数的 JSON Schema 定义。
    parameters_schema: Value,
}

struct ToolInvocation {
    call_id: String,
    tool_name: String,
    arguments: serde_json::Value,
    metadata: ToolMetadata,
}

struct ToolOutput {
    result: serde_json::Value,
}

struct AgentCoreResult<T> {
    data: Option<T>,
    error: Option<String>,
}

/// 基于函数的工具实现，适用于简单的工具逻辑。
struct FnTool {
    metadata: ToolMetadata,
    handler: Box<dyn Fn(ToolInvocation) -> AgentCoreResult<ToolOutput> + Send + Sync>,
}

impl Debug for FnTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FnTool")
            .field("metadata", &self.metadata)
            .field("handler", &"<function>")
            .finish()
    }
}

#[async_trait]
impl Tool for FnTool {
    fn metadata(&self) -> &ToolMetadata {
        &self.metadata
    }

    async fn invoke(&self, invocation: ToolInvocation) -> AgentCoreResult<ToolOutput> {
        (self.handler)(invocation)
    }
}

/// A simple library to test the build process of a Rust project.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tools: HashMap<String, Arc<dyn Tool + Send + Sync>> = HashMap::new();
    //定义一个FnTool
    let add_tool = FnTool {
        metadata: ToolMetadata {
            schema: ToolSchema {
                name: "add".to_string(),
                description: "Adds two numbers".to_string(),
                parameters_schema: serde_json::json!({"left": {"type": "number"}, "right": {"type": "number"}}),
            },
            default_visibility: "Direct".to_string(),
            capabilities: "math".to_string(),
            execution: "i dont know".to_string(),
        },
        handler: Box::new(|invocation| {
            let mut result = AgentCoreResult {
                data: None,
                error: None,
            };
            let left = invocation.arguments.get("left").and_then(|v| v.as_u64()).unwrap_or(0);
            let right = invocation.arguments.get("right").and_then(|v| v.as_u64()).unwrap_or(0);
            let fn_result = add(left, right);
            result.data = Some(ToolOutput {
                result: serde_json::json!(fn_result),
            });
            result
        }),
    };
    tools.insert(add_tool.metadata().schema.name.clone(), Arc::new(add_tool));

    return Ok(());
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
                AgentCoreResult {
                    data: Some(ToolOutput {
                        result: serde_json::json!(add(left, right)),
                    }),
                    error: None,
                }
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
        assert!(result.error.is_none());
        assert_eq!(result.data.unwrap().result, serde_json::json!(8));
    }
}
