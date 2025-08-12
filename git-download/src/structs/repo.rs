use reqwest::header::HeaderMap;
use std::env::var;

#[derive(Debug)]
pub struct Repo {
    pub api: String,
    pub path: String,
    pub headers: HeaderMap,
}

impl Repo {
    pub fn new(name: &str) -> Self {
        let api = format!("https://api.github.com/repos/{}/contents/doc", name);
        let path = format!("https://github.com/{}", name);
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/vnd.github.v3+json".parse().unwrap());
        headers.insert("User-Agent", "Flame-MCP-Server/1.0".parse().unwrap());

        if let Some(token) = var("GITHUB_API_TOKEN").ok() {
            headers.insert("Authorization", format!("token {}", token).parse().unwrap());
        }

        Self { api, path, headers }
    }
}
