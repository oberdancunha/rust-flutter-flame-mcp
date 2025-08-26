use crate::domain::resources::resource::Resource;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListResourcesOutput {
    pub jsonrpc: String,
    pub result: Resources,
}

#[derive(Debug, Serialize)]
pub struct Resources {
    pub resources: Vec<Resource>,
}
