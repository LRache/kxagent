use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ── 具体块类型 ─────────────────────────────────────────────────

/// 纯文本内容。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    pub text: String,
}

/// 工具调用内容，携带一次工具调用的完整参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallBlock {
    /// ToolInvocation 序列化后的 JSON。
    pub tool_invocation: Value,
}

/// 工具执行结果，携带工具返回的输出。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResultBlock {
    /// ToolOutput 序列化后的 JSON。
    pub tool_output: Value,
}

// ── ContentBlock 枚举 ──────────────────────────────────────────

/// 消息内容块，每个 variant 包装一种具体块类型。
///
/// 消费方通过 pattern match 即可拿到对应 struct 的强类型字段。
///
/// 序列化为 `{"type": "text", "data": {"text": "..."}}` 格式。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ContentBlock {
    /// 文本块。
    #[serde(rename = "text")]
    Text(TextBlock),

    /// 工具调用。
    #[serde(rename = "tool_call")]
    ToolCall(ToolCallBlock),

    /// 工具结果返回。
    #[serde(rename = "tool_result")]
    ToolResult(ToolResultBlock),
}

// ── AgentMessage ────────────────────────────────────────────────

/// Node 和 Tool 之间传输数据的消息结构。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// 消息来源角色（"user" | "assistant" | "tool" | "system"）。
    pub role: String,

    /// 有序内容块列表。
    pub content: Vec<ContentBlock>,

    /// 可选的附加信息。
    pub extra: Option<HashMap<String, Value>>,
}
