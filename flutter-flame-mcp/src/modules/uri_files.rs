use std::{env::current_dir, fs::read_dir, path::MAIN_SEPARATOR};

use anyhow::{Result, bail};

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
}
