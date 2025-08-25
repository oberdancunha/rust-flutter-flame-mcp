use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Snippet {
    pub uri: String,
    pub title: String,
    pub snippet: String,
}
