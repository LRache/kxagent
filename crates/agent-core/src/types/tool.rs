use super::errors::*;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use std::result::Result;

/// Tool trait，所有工具必须实现此 trait。
#[async_trait]
pub trait Tool: Debug + Send + Sync {
    /// 返回工具的元信息。
    fn metadata(&self) -> &ToolMetadata;

    /// 执行一次工具调用。
    async fn invoke(&self, invocation: ToolInvocation) -> AgentCoreResult<ToolOutput>;
}

/// 工具的元信息，包含 schema、可见性、能力和执行策略。
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSchema {
    /// 工具名称。
    pub name: String,
    /// 工具描述。
    pub description: String,
    /// 工具参数的 JSON Schema 定义。
    pub parameters_schema: Value,
}

/// 一次工具调用的请求参数。
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolOutput {
    /// 工具返回的结果，以 JSON Value 形式表示。
    pub result: serde_json::Value,
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

/// 序列化 FnTool：只序列化 metadata，跳过 handler（闭包不可序列化）。
impl Serialize for FnTool {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("FnTool", 1)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.end()
    }
}

/// 反序列化 FnTool：只反序列化 metadata，handler 置为一个
/// 会返回 InternalError 的占位闭包。
impl<'de> Deserialize<'de> for FnTool {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename = "FnTool")]
        struct Helper {
            metadata: ToolMetadata,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(FnTool {
            metadata: helper.metadata,
            handler: Box::new(|_| Err(AgentError::InternalError("deserialized FnTool has no handler".into()))),
        })
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
