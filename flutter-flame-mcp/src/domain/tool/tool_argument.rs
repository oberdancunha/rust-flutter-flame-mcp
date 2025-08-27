use rmcp::schemars;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ToolArgument {
    Topic,
    Query,
}
