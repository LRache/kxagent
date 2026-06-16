use agent_core::Tool;
use agent_core::graph::ToolNode;
use agent_core::tools::add_tool;

fn main(){
    let tool = add_tool();
    let node = ToolNode::new(tool);
    println!("{:#?}",node);
}