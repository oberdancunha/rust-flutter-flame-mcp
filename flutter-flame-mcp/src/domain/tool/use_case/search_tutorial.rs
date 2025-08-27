use std::collections::HashMap;

use crate::{
    domain::tool::tool_argument::ToolArgument, modules::documentation_files::DocumentationFiles,
};

pub struct SearchTutorial {}

impl SearchTutorial {
    pub fn execute(arguments: HashMap<ToolArgument, String>) -> String {
        let mut result = String::new();

        if let Some(topic_value) = arguments.get(&ToolArgument::Topic) {
            if topic_value.is_empty() {
                result = "❌ Tutorial topic cannot be empty".into();
            } else {
                result = DocumentationFiles::handle_tutorial_request(topic_value);
            }
        }

        result
    }
}
