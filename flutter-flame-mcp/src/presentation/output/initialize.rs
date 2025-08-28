use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Resources {
    #[serde(rename = "listChanged")]
    pub list_changed: bool,
}

#[derive(Debug, Serialize)]
pub struct Tools {
    #[serde(rename = "listChanged")]
    pub list_changed: bool,
}

#[derive(Debug, Serialize)]
pub struct Capabilities {
    pub resources: Resources,
    pub tools: Tools,
}

#[derive(Debug, Serialize)]
pub struct Result {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: Capabilities,
}

#[derive(Debug, Serialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct InitializeOutput {
    pub jsonrpc: String,
    pub result: Result,
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}
