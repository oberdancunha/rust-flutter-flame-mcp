use std::{collections::HashMap, fmt::Write};

use crate::domain::tool::{
    repository::tool_repository::ToolRepository, tool_argument::ToolArgument,
};

pub struct SearchDocumentation {}

impl SearchDocumentation {
    pub fn execute(arguments: HashMap<ToolArgument, String>) -> String {
        let mut result = String::new();
        if let Some(query_value) = arguments.get(&ToolArgument::Query) {
            if query_value.is_empty() {
                result = "❌ Search query cannot be empty".into();
            } else {
                let tool_repository = ToolRepository::new();
                let results = tool_repository.search(&query_value);
                if results.is_empty() {
                    result = format!("No results found for {}", query_value);
                } else {
                    let mut buffer = String::new();
                    for result in results.iter().take(5) {
                        writeln!(&mut buffer, "📄 **{}** ({})", result.title, result.uri).unwrap();
                        writeln!(&mut buffer, "   {}", result.snippet).unwrap();
                    }
                    writeln!(&mut result, "{}", buffer).unwrap();
                }
            }
        }

        result
    }
}
