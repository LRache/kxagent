//! TODO: Add a description of the library here.
//! This an agent core library, which provides the basic functionalities for the agent.

#![deny(missing_docs)]

use async_trait::async_trait;
use serde_json::Value;
use std::fmt::Debug;
use  std::result::Result;

/// Tool trait，所有工具必须实现此 trait。
#[async_trait]
pub trait Tool: Debug + Send + Sync {
    /// 返回工具的元信息。
    fn metadata(&self) -> &ToolMetadata;

    /// 执行一次工具调用。
    async fn invoke(&self, invocation: ToolInvocation) -> AgentCoreResult<ToolOutput>;
}

/// 工具的元信息，包含 schema、可见性、能力和执行策略。
#[derive(Debug, Clone)]
pub struct ToolMetadata {
    /// 工具的 JSON Schema 定义。
    pub schema: ToolSchema,
    /// Agent 未显式配置时使用的默认可见性。
    pub default_visibility: String,
    /// 工具能力标记。
    pub capabilities: String,
    /// 超时、可中断性和结果返回策略。
    pub execution: String,
}

/// 工具的 Schema 定义，包含名称、描述和参数 JSON Schema。
#[derive(Debug, Clone)]
pub struct ToolSchema {
    /// 工具名称。
    pub name: String,
    /// 工具描述。
    pub description: String,
    /// 工具参数的 JSON Schema 定义。
    pub parameters_schema: Value,
}

/// 一次工具调用的请求参数。
#[derive(Debug)]
pub struct ToolInvocation {
    /// 调用的唯一标识。
    pub call_id: String,
    /// 被调用的工具名称。
    pub tool_name: String,
    /// 工具参数，以 JSON Value 形式传递。
    pub arguments: serde_json::Value,
    /// 工具元信息。
    pub metadata: ToolMetadata,
}

/// 工具调用的输出结果。
#[derive(Debug)]
pub struct ToolOutput {
    /// 工具返回的结果，以 JSON Value 形式表示。
    pub result: serde_json::Value,
}


/// Agent 错误类型，包含各种可能的错误情况，如工具调用失败、参数错误等。
#[derive(Debug)]
pub enum AgentError {
    /// 工具未找到。
    ToolNotFound(String),
    /// 参数无效。
    InvalidArguments(String),
    /// 工具执行失败。
    ExecutionFailed(String),
    /// 权限不足。
    PermissionDenied(String),
    /// 执行超时。
    Timeout(String),
    /// 内部错误。
    InternalError(String),
}


/// Agent 核心结果的泛型封装。
pub type AgentCoreResult<T> = Result<T, AgentError>;

/// 基于函数的工具实现，适用于简单的工具逻辑。
pub struct FnTool {
    /// 工具元信息。
    pub metadata: ToolMetadata,
    /// 工具的处理函数。
    pub handler: Box<dyn Fn(ToolInvocation) -> AgentCoreResult<ToolOutput> + Send + Sync>,
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
