use std::collections::HashMap;

use crate::domain::tool::{
    repository::tool_repository::ToolRepository, tool_argument::ToolArgument,
};

pub struct SearchTutorial {}

impl SearchTutorial {
    pub fn execute(arguments: HashMap<ToolArgument, String>) -> String {
        let mut result = String::new();

        if let Some(topic_value) = arguments.get(&ToolArgument::Topic) {
            if topic_value.is_empty() {
                result = "❌ Tutorial topic cannot be empty".into();
            } else {
                let tool_repository = ToolRepository::new();
                result = tool_repository.handle_tutorial_request(topic_value);
            }
        }

        result
    }
}
