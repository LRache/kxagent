use agent_core::AgentError;

fn main() -> Result<(), AgentError> {
    let e = AgentError::Timeout("这是一个错误".to_string());
    return Err(e);
}
