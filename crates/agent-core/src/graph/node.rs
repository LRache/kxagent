#![deny(missing_docs)]


use crate::{Tool, ToolInvocation, types::{AgentError, FnTool, agentic_message::{AgentMessage,ContentBlock}}
};
use serde_json::Value;
use async_trait::async_trait;

/// Node 的 trait，定义节点的完整生命周期。
#[async_trait]
pub trait BaseNode:Send + Sync {

    /// 创建新节点。
    fn new() -> Self
    where
        Self: Sized;
    /// 数据预处理,暂时将接受的message类型定为json
    fn prep(&self,message:AgentMessage<Value>)->Result<Value,AgentError>;
    /// 执行逻辑，可重写。
    async fn exec(&self,message:Value)->Result<Value,AgentError>;
    /// 数据后处理（结果存储、日志记录等）。
    fn post(&self,message:Value)->AgentMessage<Value>;
    /// 框架执行的逻辑，不可重写，保证流程一致性。
    async  fn _exec(&self,message:Value) ->Result<Value,AgentError>{
        return self.exec(message).await;
    }
    /// Node 执行的完整流程。内部直接直接通过json传递数据，接受与传出数据时要用agentic_message包裹
    async fn _run(&self,message:AgentMessage<Value>) ->Result<AgentMessage<Value>,AgentError>{
         
        match  self.prep(message){
            Ok( data)=>{
                match self._exec(data).await{
                    Ok(data) =>{
                        let message = self.post(data);
                        return Ok(message);
                    }
                    Err(e) => return Err(e),
                }
            }
            Err(e)=> return Err(e),
        }
    }
    /// 连接下一个 node。
    fn next(&mut self, node: Box<dyn BaseNode>);
}

///这是一个toolNode，用来调用tool，同时指向下一个node
pub struct ToolNode {
    /// 该节点对应的工具。
    tool: FnTool,
    /// 可选的下一个节点。
    next_node: Option<Box<dyn BaseNode>>,
}

#[async_trait]
impl BaseNode for ToolNode {
    fn new() -> Self {
        // 这里可以根据需要添加默认工具或参数。
        return Self {
            tool: FnTool {
                metadata: crate::ToolMetadata {
                    schema: crate::ToolSchema {
                        name: "default".to_string(),
                        description: "Default tool".to_string(),
                        parameters_schema: serde_json::json!({}),
                    },
                    default_visibility: "Direct".to_string(),
                    capabilities: "default".to_string(),
                    execution: "sync".to_string(),
                },
                handler: Box::new(|_invocation| {
                    return crate::AgentCoreResult::Ok(crate::ToolOutput {
                        result: serde_json::json!(null),
                    });
                }),
            },
            next_node: None,
        };
    }

    //message是一条llm call
    fn prep(&self,message:AgentMessage<Value>)->Result<Value,AgentError> {
        match message.content[0].data.get("ToolInvocation") {
            Some(data) => Ok(data.clone()) ,
            None =>  return Err(AgentError::InvalidArguments("None arguments".to_string())),
        }         
    }

    async fn exec(&self,message:Value)->Result<Value,AgentError> {
        let  inv:ToolInvocation = serde_json::from_value(message)
        .map_err(|e| AgentError::InvalidArguments(format!("parse failed: {e}")))?;
        
        let output = self.tool.invoke(inv).await?;

        return Ok(serde_json::json!(output));
    }

    fn post(&self, message: Value) -> AgentMessage<Value> {
        AgentMessage {
            role: String::new(),
            content: vec![ContentBlock {
                data_type: "tool_result".to_string(),
                data: message,
            }],
            extra: None,
        }
    }

    fn next(&mut self, node: Box<dyn BaseNode>) {
        self.next_node = Some(node);
    }
}