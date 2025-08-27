use std::fmt::Write;

use crate::{data_source::docs_cache_data_source::DocsCacheDataSource, structs::snippet::Snippet};

pub struct ToolRepository {
    docs_cache_data_source: DocsCacheDataSource,
}

impl ToolRepository {
    pub fn new() -> Self {
        Self {
            docs_cache_data_source: DocsCacheDataSource::new(),
        }
    }

    pub fn search(&self, query: &str) -> Vec<Snippet> {
        let mut results: Vec<Snippet> = vec![];
        for uri in self.docs_cache_data_source.files_uri.iter() {
            let content = DocsCacheDataSource::get_content(&uri).unwrap();
            if !content.is_empty() && content.to_lowercase().contains(&query.to_lowercase()) {
                let title = uri.replace("flame://", "").replace('/', " > ");
                let snippet: String = DocsCacheDataSource::extract_snippet(&content, query);
                results.push(Snippet {
                    uri: uri.clone(),
                    title,
                    snippet,
                });
            }
        }

        results
    }

    pub fn handle_tutorial_request(&self, topic: &str) -> String {
        let lower_topic = topic.to_lowercase();
        if lower_topic == "list" {
            return self.docs_cache_data_source.list_all_tutorials();
        }
        if lower_topic.contains("space shooter") || lower_topic.contains("spaceshooter") {
            return self
                .docs_cache_data_source
                .get_complete_tutorial("space_shooter");
        } else if lower_topic.contains("platformer") {
            return self
                .docs_cache_data_source
                .get_complete_tutorial("platformer");
        } else if lower_topic.contains("klondike") {
            return self
                .docs_cache_data_source
                .get_complete_tutorial("klondike");
        }
        let tutorial_results = self.docs_cache_data_source.search_tutorials(&lower_topic);
        if tutorial_results.is_empty() {
            return format!(
                "No tutorial found for {}. Try \"list\" to see all available tutorials.",
                topic
            );
        }
        let mut buffer = String::new();
        writeln!(
            &mut buffer,
            "🎓 Found {} tutorial(s) for {}:\n",
            tutorial_results.len(),
            topic
        )
        .unwrap();
        for tutorial in tutorial_results {
            writeln!(&mut buffer, "📚 **{}* ({})", tutorial.title, tutorial.uri).unwrap();
            writeln!(&mut buffer, "   {}\n", tutorial.snippet).unwrap();
        }

        buffer
    }
}
