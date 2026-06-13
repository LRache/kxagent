use std::fmt;

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

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::ToolNotFound(msg) => write!(f, "tool not found: {msg}"),
            AgentError::InvalidArguments(msg) => write!(f, "invalid arguments: {msg}"),
            AgentError::ExecutionFailed(msg) => write!(f, "execution failed: {msg}"),
            AgentError::PermissionDenied(msg) => write!(f, "permission denied: {msg}"),
            AgentError::Timeout(msg) => write!(f, "timeout: {msg}"),
            AgentError::InternalError(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl std::error::Error for AgentError {}
