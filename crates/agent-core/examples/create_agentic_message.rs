use agent_core::types::{AgentMessage, ContentBlock};
use serde_json::{json, Value};
use std::collections::HashMap;

fn main() {
    // 1. 最简单的构造：一条 text 消息
    let message1 = AgentMessage {
        role: "user".to_string(),
        content: vec![ContentBlock {
            data_type: "text".to_string(),
            data: json!("Hello, agent!"),
        }],
        extra: None,
    };

    println!("message1:{:#?}", message1);

    // 2. 带 extra 的复杂消息
    let mut extra = HashMap::new();
    extra.insert("session_id".to_string(), json!("abc-123"));

    let rich_message = AgentMessage {
        role: "assistant".to_string(),
        content: vec![
            ContentBlock {
                data_type: "text".to_string(),
                data: json!("我来调用工具"),
            },
            ContentBlock {
                data_type: "tool_call".to_string(),
                data: json!({"name": "add", "args": {"left": 3, "right": 5}}),
            },
        ],
        extra: Some(extra),
    };

    println!("rich_message:{:#?}", rich_message);
}