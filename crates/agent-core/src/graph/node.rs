#![deny(missing_docs)]

use crate::{
    Tool, ToolInvocation,
    types::{
        AgentError, FnTool,
        agentic_message::{AgentMessage, ContentBlock, ToolResultBlock},
    },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Node 的 trait，定义节点的完整生命周期。
#[async_trait]
pub trait BaseNode: Send + Sync + std::fmt::Debug {
    /// 数据预处理：从入站消息中提取本节点需要的数据。
    fn prep(&self, message: AgentMessage) -> Result<Value, AgentError>;

    /// 核心执行逻辑，可被子类重写。
    async fn exec(&self, data: Value) -> Result<Value, AgentError>;

    /// 数据后处理：将执行结果包装为出站消息。
    fn post(&self, result: Value) -> AgentMessage;

    /// 框架内部执行，默认委托给 [`exec`](BaseNode::exec)。
    async fn _exec(&self, data: Value) -> Result<Value, AgentError> {
        self.exec(data).await
    }

    /// Node 执行的完整流程：prep → _exec → post。
    async fn _run(&self, message: AgentMessage) -> Result<AgentMessage, AgentError> {
        let data = self.prep(message)?;
        let result = self._exec(data).await?;
        Ok(self.post(result))
    }

    /// 连接下一个 node。
    fn next(&mut self, node: Box<dyn BaseNode>);
}

/// 工具调用节点：入站消息中提取 ToolCall 块，执行工具，将结果包装为 ToolResult。
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolNode {
    /// 该节点对应的工具。
    tool: FnTool,
    /// 可选的下一个节点。
    #[serde(skip)]
    next_node: Option<Box<dyn BaseNode>>,
}

#[async_trait]
impl BaseNode for ToolNode {
    fn prep(&self, message: AgentMessage) -> Result<Value, AgentError> {
        let block = message
            .content
            .first()
            .ok_or_else(|| AgentError::InvalidArguments("empty content".into()))?;
        match block {
            ContentBlock::ToolCall(tc) => Ok(tc.tool_invocation.clone()),
            ContentBlock::Text(_) => Err(AgentError::InvalidArguments("expected ToolCall block, got Text".into())),
            ContentBlock::ToolResult(_) => Err(AgentError::InvalidArguments(
                "expected ToolCall block, got ToolResult".into(),
            )),
        }
    }

    async fn exec(&self, data: Value) -> Result<Value, AgentError> {
        let inv: ToolInvocation = serde_json::from_value(data)
            .map_err(|e| AgentError::InvalidArguments(format!("parse ToolInvocation failed: {e}")))?;

        let output = self.tool.invoke(inv).await?;

        Ok(serde_json::json!(output))
    }

    fn post(&self, result: Value) -> AgentMessage {
        AgentMessage {
            role: "tool".into(),
            content: vec![ContentBlock::ToolResult(ToolResultBlock { tool_output: result })],
            extra: None,
        }
    }

    fn next(&mut self, node: Box<dyn BaseNode>) {
        self.next_node = Some(node);
    }
}

impl ToolNode {
    /// 传入一个 FnTool，创建一个 ToolNode。
    pub fn new(tool: FnTool) -> Self {
        Self { tool, next_node: None }
    }
}
