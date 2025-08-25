use std::{env::current_dir, fs::read_dir, path::MAIN_SEPARATOR};

use anyhow::{Result, bail};

use crate::structs::snippet::Snippet;

#[derive(Debug)]
pub struct UriFiles {
    pub uri: String,
}

impl UriFiles {
    pub fn build_index() -> Result<Vec<Self>> {
        let current_dir = current_dir()?;
        let docs_cache_dir = current_dir.join("docs_cache");
        let mut stack = vec![docs_cache_dir];
        let mut files: Vec<Self> = vec![];
        while let Some(path) = stack.pop() {
            if let Ok(entries) = read_dir(&path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        stack.push(entry_path);
                    } else {
                        let entry_path_extension = entry_path.extension().unwrap();
                        if entry_path.is_file() && entry_path_extension == "md" {
                            let file = entry_path.strip_prefix(&current_dir)?;
                            let file = file
                                .to_str()
                                .unwrap()
                                .replace(MAIN_SEPARATOR, "/")
                                .replace(".md", "");
                            let uri = format!("flame://{}", file);
                            files.push(Self { uri });
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    pub fn get_content(uri: &str) -> Result<String> {
        let uri = uri.replace("flame://", "");
        let current_dir = current_dir()?;
        let file = current_dir.join(format!("{}.md", uri));
        if file.is_file() {
            let content = std::fs::read_to_string(&file)?;
            return Ok(content);
        } else {
            bail!("Arquivo não encontrado: {}", file.display());
        }
    }

    pub fn search(query: &str) -> Vec<Snippet> {
        let mut results: Vec<Snippet> = vec![];
        let resources = Self::build_index().unwrap();
        for resource in resources {
            let content = Self::get_content(&resource.uri).unwrap();
            if !content.is_empty() && content.to_lowercase().contains(&query.to_lowercase()) {
                let title = resource.uri.replace("flame://", "").replace('/', " > ");
                let snippet = Self::_extract_snippet(&content, query);
                results.push(Snippet {
                    uri: resource.uri,
                    title,
                    snippet,
                });
            }
        }

        results
    }

    fn _extract_snippet(content: &str, query: &str) -> String {
        let lines: Vec<&str> = content.split("\n").collect();
        for i in 0..lines.len() {
            if lines[i].to_lowercase().contains(&query.to_lowercase()) {
                let start: usize = i.saturating_sub(1);
                let end: usize = std::cmp::min(lines.len(), i.saturating_add(2));
                return lines[start..end].join("\n").trim().into();
            }
        }
        return lines
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
    }
}
