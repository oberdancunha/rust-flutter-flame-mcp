use serde_json::{Value, to_value};

use crate::domain::tools::{tools_query, tools_topic};

pub struct ListTools {}

impl ListTools {
    pub fn execute() -> Vec<Value> {
      vec![
            to_value(&tools_query::ToolsQuery {
                name: "search_documentation".into(),
                description: "Search through Flame documentation".into(),
                input_schema: tools_query::InputSchema {
                    type_schema: "object".into(),
                    properties: tools_query::Properties {
                        query: tools_query::Query {
                            type_query: "string".into(),
                            description: "Search query".into(),
                        },
                    },
                    required: vec!["query".into()],
                },
            })
            .unwrap(),
            to_value(&tools_topic::ToolsTopic {
                name: "tutorial".into(),
                description: "Get complete Flame tutorials with step-by-step instructions for building games (space shooter, platformer, klondike). Use this for learning how to build specific games.".into(),
                input_schema: tools_topic::InputSchema {
                    type_schema: "object".into(),
                    properties: tools_topic::Properties {
                        topic: tools_topic::Topic {
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
