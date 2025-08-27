use std::{
    cmp::Ordering::{Greater, Less},
    collections::HashMap,
    env::current_dir,
    fs::read_dir,
    path::MAIN_SEPARATOR,
};

use anyhow::{Result, bail};
use regex::Regex;
use std::fmt::Write;

use crate::structs::snippet::Snippet;

#[derive(Debug)]
pub struct DocsCacheDataSource {
    pub files_uri: Vec<String>,
}

impl DocsCacheDataSource {
    pub fn new() -> Self {
        let current_dir = current_dir().unwrap();
        let docs_cache_dir = current_dir.join("docs_cache");
        let mut stack = vec![docs_cache_dir];
        let mut files_uri: Vec<String> = vec![];
        while let Some(path) = stack.pop() {
            if let Ok(entries) = read_dir(&path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        stack.push(entry_path);
                    } else {
                        let entry_path_extension = entry_path.extension().unwrap();
                        if entry_path.is_file() && entry_path_extension == "md" {
                            let file = entry_path.strip_prefix(&current_dir).unwrap();
                            let file = file
                                .to_str()
                                .unwrap()
                                .replace(MAIN_SEPARATOR, "/")
                                .replace(".md", "");
                            let uri = format!("flame://{}", file);
                            files_uri.push(uri);
                        }
                    }
                }
            }
        }

        Self { files_uri }
    }

    pub fn list_all_tutorials(&self) -> String {
        let mut tutorial_groups: HashMap<String, Vec<String>> = HashMap::new();
        let tutorials: Vec<String> = self._get_tutorials();
        if tutorials.is_empty() {
            return "No tutorials found in the documentation cache.".into();
        }
        let mut buffer = String::new();
        writeln!(&mut buffer, "🎓 Available Flame Tutorials:\n").unwrap();
        for uri in tutorials {
            // Parse URI like "flame://tutorials/space_shooter/step_1"
            let path = uri.replace("flame://", "");
            let parts: Vec<&str> = path.split('/').collect();
            if parts.len() >= 2 && parts[0] == "tutorials" {
                let main_topic = if parts.len() >= 3 {
                    parts[1]
                } else {
                    "general"
                };
                tutorial_groups
                    .entry(main_topic.to_owned())
                    .or_default()
                    .push(uri);
            }
        }

        for (topic, uris) in tutorial_groups.iter_mut() {
            writeln!(&mut buffer, "📖 **{}**\n", topic).unwrap();
            uris.sort_by(|a, b| {
                let a_name = a.split('/').last().unwrap();
                let b_name = b.split('/').last().unwrap();

                // Main tutorial files (same name as directory) come first
                if a_name == topic.as_str() {
                    return Less;
                }
                if b_name == topic.as_str() {
                    return Greater;
                }
                a_name.cmp(b_name)
            });

            for uri in uris {
                let title = uri.replace("flame://", "").replace('/', " > ");
                writeln!(&mut buffer, "   • {}", title).unwrap();
            }
            writeln!(&mut buffer, "\n").unwrap();
        }
        writeln!(
            &mut buffer,
            "💡 Use `tutorial <topic>` to get specific tutorial content."
        )
        .unwrap();
        writeln!(
            &mut buffer,
            "   Example: `tutorial space shooter` or `tutorial platformer`'"
        )
        .unwrap();

        buffer
    }

    pub fn get_complete_tutorial(&self, tutorial_name: &str) -> String {
        let mut tutorial_resources: Vec<String> = self
            .files_uri
            .iter()
            .cloned()
            .filter(|uri| uri.contains(&format!("tutorials/{}", tutorial_name)))
            .collect();
        if tutorial_resources.is_empty() {
            return format!("No tutorial found for {}", tutorial_name);
        }
        tutorial_resources.sort_by(|a, b| {
            let a_name = a.split('/').last().unwrap();
            let b_name = b.split('/').last().unwrap();

            // Sort to get main tutorial first, then steps in order
            if a_name == tutorial_name {
                return Less;
            }
            if b_name == tutorial_name {
                return Greater;
            }

            // Sort steps numerically
            let a_step = Self::_extract_step_number(a_name);
            let b_step = Self::_extract_step_number(b_name);

            return a_step.cmp(&b_step);
        });
        let mut buffer = String::new();
        writeln!(
            &mut buffer,
            "🎮 {} Tutorial - Complete Guide\n",
            Self::_format_topic_name(tutorial_name)
        )
        .unwrap();
        writeln!(&mut buffer, "{}", "=".repeat(50)).unwrap();
        writeln!(&mut buffer, "\n").unwrap();
        for i in 0..tutorial_resources.len() {
            let uri = &tutorial_resources[i];
            let content = Self::get_content(uri).unwrap();
            if !content.is_empty() {
                let file_name = &uri.split('/').last().unwrap();
                let is_main_in_tutorial = file_name == &tutorial_name;
                let step_number = if is_main_in_tutorial {
                    0
                } else {
                    Self::_extract_step_number(&file_name)
                };
                if is_main_in_tutorial {
                    writeln!(&mut buffer, "📖 **Overview**\n").unwrap();
                } else {
                    writeln!(&mut buffer, "📝 **Step {}**\n", step_number).unwrap();
                }
                // Get first few paragraphs of content
                let lines: Vec<&str> = content.split('\n').collect();
                let content_lines: Vec<&str> = lines
                    .into_iter()
                    .filter(|line| {
                        !line.trim().is_empty()
                            && !line.starts_with("```")
                            && !line.starts_with("![")
                            && !line.starts_with("{")
                    })
                    .take(10)
                    .collect();
                for line in content_lines {
                    if line.starts_with('#') {
                        writeln!(&mut buffer, "**{}**", line.replace('#', "")).unwrap();
                    } else {
                        writeln!(&mut buffer, "{}", line).unwrap();
                    }
                }
                writeln!(&mut buffer, "\n📄 Full content: {}", uri).unwrap();
                writeln!(&mut buffer, "{}", &"-".repeat(30)).unwrap();
                writeln!(&mut buffer, "\n").unwrap();
            }
        }

        writeln!(&mut buffer, "💡 **Next Steps:**").unwrap();
        writeln!(
            &mut buffer,
            "• Use the URIs above to get full content for each step"
        )
        .unwrap();
        writeln!(&mut buffer, "• Follow the steps in order for best results").unwrap();
        writeln!(&mut buffer, "• Each step builds upon the previous one").unwrap();

        buffer
    }

    pub fn search_tutorials(&self, query: &str) -> Vec<Snippet> {
        let mut results: Vec<Snippet> = vec![];
        let tutorial_resources: Vec<String> = self._get_tutorials();
        for uri in tutorial_resources {
            let content = Self::get_content(&uri).unwrap();
            if !content.is_empty() && content.to_lowercase().contains(&query.to_lowercase()) {
                let title = uri.replace("flame://", "").replace("/", " > ");
                let snippet = Self::extract_snippet(&content, query);
                results.push(Snippet {
                    uri,
                    title,
                    snippet,
                });
            }
        }

        results
    }

    fn _get_tutorials(&self) -> Vec<String> {
        self.files_uri
            .iter()
            .filter(|uri| uri.contains("tutorials/"))
            .cloned()
            .collect()
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

    pub fn extract_snippet(content: &str, query: &str) -> String {
        let lines: Vec<&str> = content.split('\n').collect();
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

    fn _extract_step_number(file_name: &str) -> u16 {
        let re: Regex = Regex::new(r"step_?(\d+)").unwrap();
        let step_number = re
            .captures(file_name)
            .and_then(|caps| caps.get(1))
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(999);

        step_number
    }

    fn _format_topic_name(topic: &str) -> String {
        let topic_formatted = topic
            .split(|c: char| c == '_' || c == '-' || c.is_whitespace())
            .filter(|word| !word.is_empty())
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        topic_formatted
    }
}
