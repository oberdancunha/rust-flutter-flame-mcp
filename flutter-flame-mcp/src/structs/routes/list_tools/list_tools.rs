use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Result {
    pub tools: Vec<Value>,
}

#[derive(Debug, Serialize)]
pub struct ListToolsOutput {
    pub jsonrpc: f32,
    pub result: Result,
}
