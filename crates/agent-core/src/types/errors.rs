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
