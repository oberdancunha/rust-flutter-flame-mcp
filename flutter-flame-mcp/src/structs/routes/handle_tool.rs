use rmcp::schemars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::domain::tools::tool_argument::ToolArgument;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct HandleToolInput {
    pub name: ToolName,
    pub arguments: HashMap<ToolArgument, String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ToolName {
    SearchDocumentation,
    Tutorial,
}

#[derive(Debug, Serialize)]
pub struct Result {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct HandleToolOutput {
    pub jsonrpc: String,
    pub result: Result,
}
