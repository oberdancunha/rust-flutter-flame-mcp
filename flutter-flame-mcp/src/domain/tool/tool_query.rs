use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Query {
    #[serde(rename = "type")]
    pub type_query: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct Properties {
    pub query: Query,
}

#[derive(Debug, Serialize)]
pub struct InputSchema {
    #[serde(rename = "type")]
    pub type_schema: String,
    pub properties: Properties,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ToolQuery {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: InputSchema,
}
