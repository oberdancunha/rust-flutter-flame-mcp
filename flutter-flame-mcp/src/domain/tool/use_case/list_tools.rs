use serde_json::{Value, to_value};

use crate::domain::tool::{tool_query, tool_topic};

pub struct ListTools {}

impl ListTools {
    pub fn execute() -> Vec<Value> {
      vec![
            to_value(&tool_query::ToolQuery {
                name: "search_documentation".into(),
                description: "Search through Flame documentation".into(),
                input_schema: tool_query::InputSchema {
                    type_schema: "object".into(),
                    properties: tool_query::Properties {
                        query: tool_query::Query {
                            type_query: "string".into(),
                            description: "Search query".into(),
                        },
                    },
                    required: vec!["query".into()],
                },
            })
            .unwrap(),
            to_value(&tool_topic::ToolTopic {
                name: "tutorial".into(),
                description: "Get complete Flame tutorials with step-by-step instructions for building games (space shooter, platformer, klondike). Use this for learning how to build specific games.".into(),
                input_schema: tool_topic::InputSchema {
                    type_schema: "object".into(),
                    properties: tool_topic::Properties {
                        topic: tool_topic::Topic {
                            type_topic: "string".into(),
                            description: 
                            "Tutorial topic: \"space shooter\" for complete space shooter game tutorial, \"platformer\" for platformer game tutorial, \"klondike\" for card game tutorial, or \"list\" to see all available tutorials".into(),
                        },
                    },
                    required: vec!["topic".into()],
                },
            })
            .unwrap(),
        ]
    }
}
