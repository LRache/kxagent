// 此示例演示如何使用 agent-core 库中预置的工具（无需重复定义）。

use agent_core::{Tool, tools};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 直接从库中获取预置工具
    let factorial = tools::factorial_tool();

    // ---- 使用 factorial 工具 ----
    let inv = agent_core::ToolInvocation {
        call_id: "use-1".to_string(),
        tool_name: "factorial".to_string(),
        arguments: serde_json::json!({"number": 6}),
        metadata: factorial.metadata().clone(),
    };

    let output = factorial.invoke(inv).await?;
    println!("factorial(6) = {}", output.result); // 720

    Ok(())
}
