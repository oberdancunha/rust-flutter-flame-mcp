use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct HandleResourceInput {
    pub uri: String,
}

#[derive(Debug, Serialize)]
pub struct Contents {
    pub uri: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct Result {
    pub contents: Contents,
}

#[derive(Debug, Serialize)]
pub struct HandleResourceOutput {
    pub jsonrpc: f32,
    pub result: Result,
}
