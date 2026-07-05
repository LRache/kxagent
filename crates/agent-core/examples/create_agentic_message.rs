use agent_core::types::agentic_message::{AgentMessage, ContentBlock, TextBlock, ToolCallBlock};
use serde_json::json;
use std::collections::HashMap;

fn main() {
    // 1. 最简单的构造：一条纯文本消息
    let message1 = AgentMessage {
        role: "user".to_string(),
        content: vec![ContentBlock::Text(TextBlock {
            text: "Hello, agent!".into(),
        })],
        extra: None,
    };

    println!("message1: {:#?}", message1);

    // 2. 带 extra 和多块内容的消息
    let mut extra = HashMap::new();
    extra.insert("session_id".to_string(), json!("abc-123"));

    let rich_message = AgentMessage {
        role: "assistant".to_string(),
        content: vec![
            ContentBlock::Text(TextBlock {
                text: "我来调用工具".into(),
            }),
            ContentBlock::ToolCall(ToolCallBlock {
                tool_invocation: json!({"name": "add", "args": {"left": 3, "right": 5}}),
            }),
        ],
        extra: Some(extra),
    };

    println!("rich_message: {:#?}", rich_message);
}
