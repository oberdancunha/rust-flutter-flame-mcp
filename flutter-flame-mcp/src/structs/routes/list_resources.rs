use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

#[derive(Debug, Serialize)]
pub struct Resources {
    pub resources: Vec<Resource>,
}

#[derive(Debug, Serialize)]
pub struct ListResourcesOutput {
    pub jsonrpc: String,
    pub result: Resources,
}
