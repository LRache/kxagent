#![deny(missing_docs)]

use crate::{Tool, types::FnTool};


/// Node 的 trait，定义节点的完整生命周期。
pub trait BaseNode {
    /// 创建新节点。
    fn new() -> Self
    where
        Self: Sized;
    /// 数据预处理。
    fn prep(&self);
    /// 执行逻辑，可重写。
    fn exec(&self);
    /// 数据后处理（结果存储、日志记录等）。
    fn post(&self);
    /// 框架执行的逻辑，不可重写，保证流程一致性。
    fn _exec(&self) {
        return self.exec();
    }
    /// Node 执行的完整流程。
    fn _run(&self) {
        self.prep();
        self._exec();
        self.post();
    }
    /// 连接下一个 node。
    fn next(&mut self, node: Box<dyn BaseNode>);
}

struct ToolNode {
    /// 该节点对应的工具。
    tool: FnTool,
    /// 可选的下一个节点。
    next_node: Option<Box<dyn BaseNode>>,
}

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

    fn prep(&self) {
        // 这里可以添加工具调用前的准备工作，如参数验证、日志记录等。
        
    }

    fn exec(&self) {
        // 这里执行工具的核心逻辑，可以调用 self.tool.handler。
    }

    fn post(&self) {
        // 这里可以添加工具调用后的处理工作，如结果存储、日志记录等。
    }

    fn next(&mut self, node: Box<dyn BaseNode>) {
        self.next_node = Some(node);
    }
}