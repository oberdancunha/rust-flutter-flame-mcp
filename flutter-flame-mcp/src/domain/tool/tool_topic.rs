use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Topic {
    #[serde(rename = "type")]
    pub type_topic: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct Properties {
    pub topic: Topic,
}

#[derive(Debug, Serialize)]
pub struct InputSchema {
    #[serde(rename = "type")]
    pub type_schema: String,
    pub properties: Properties,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ToolTopic {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: InputSchema,
}
