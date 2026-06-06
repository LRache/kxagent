use serde_json::Value;
use std::collections::HashMap;

///node和tool直接传输数据的结构
pub struct AgentMessage<T> {
    ///信息来源
    pub role: String,
    ///内容
    pub content: Vec<ContentBlock<T>>,
    // pub agentResonseMetadata: Option<agentResponseMetadata>,
    ///或许会有的额外内容
    pub extra: Option<HashMap<String, Value>>,
}

///包装message传输的data
pub struct ContentBlock<T> {
    ///type
    pub _type: String,
    ///data
    pub data: T,
}
