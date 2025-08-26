use crate::{
    domain::resources::{list_resources::ListResources, handle_resources::HandleResources},
    modules::{documentation_files::DocumentationFiles},
    structs::routes::{
        handle_resource, initialize, list_resources, handle_tool,
        list_tools::{list_tools, list_tools_query, list_tools_topic},
    },
};
use std::{fmt::Write};
use rmcp::{
    ServerHandler,
    handler::server::tool::{Parameters, ToolRouter},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};
use serde_json::{Value, to_string, to_value};

#[derive(Debug, Clone)]
pub struct Features {
    pub tool_router: ToolRouter<Self>,
}

#[tool_router]
impl Features {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Initialize server")]
    fn initialize(&self) -> String {
        to_string(&initialize::InitializeOutput {
            jsonrpc: "2.0".into(),
            result: initialize::Result {
                protocol_version: "2024-11-05".into(),
                capabilities: initialize::Capabilities {
                    resources: initialize::Resources { list_changed: true },
                    tools: initialize::Tools { list_changed: true },
                },
            },
            server_info: initialize::ServerInfo {
                name: "flutter-flame-mcp".into(),
                version: "1.0.0".into(),
                description: "Flame game engine MCP server with on-demand GitHub documentation"
                    .into(),
            },
        })
        .unwrap()
    }

    #[tool(description = "List resources available")]
    fn list_resources(&self) -> String {
        let resources = ListResources::execute();

        to_string(&list_resources::ListResourcesOutput {
            jsonrpc: "2.0".into(),
            result: list_resources::Resources { resources },
        })
        .unwrap()
    }

    #[tool(description = "Handle a specific resource")]
    fn handle_resource(
        &self,
        Parameters(handle_resource::HandleResourceInput { uri }): Parameters<
            handle_resource::HandleResourceInput,
        >,
    ) -> String {
        let content = HandleResources::execute(&uri);
        to_string(&handle_resource::HandleResourceOutput {
            jsonrpc: "2.0".into(),
            result: handle_resource::Result {
                contents: handle_resource::Contents {
                    uri,
                    mime_type: "text/markdown".into(),
                    text: content,
                },
            },
        })
        .unwrap()
    }

    #[tool(description = "List tools available")]
    fn list_tools(&self) -> String {
        let tools: Vec<Value> = vec![
            to_value(&list_tools_query::ListToolsQueryOutput {
                name: "search_documentation".into(),
                description: "Search through Flame documentation".into(),
                input_schema: list_tools_query::InputSchema {
                    type_schema: "object".into(),
                    properties: list_tools_query::Properties {
                        query: list_tools_query::Query {
                            type_query: "string".into(),
                            description: "Search query".into(),
                        },
                    },
                    required: vec!["query".into()],
                },
            })
            .unwrap(),
            to_value(&list_tools_topic::ListToolsTopicOutput {
                name: "tutorial".into(),
                description: "Get complete Flame tutorials with step-by-step instructions for building games (space shooter, platformer, klondike). Use this for learning how to build specific games.".into(),
                input_schema: list_tools_topic::InputSchema {
                    type_schema: "object".into(),
                    properties: list_tools_topic::Properties {
                        topic: list_tools_topic::Topic {
                            type_topic: "string".into(),
                            description: 
                            "Tutorial topic: \"space shooter\" for complete space shooter game tutorial, \"platformer\" for platformer game tutorial, \"klondike\" for card game tutorial, or \"list\" to see all available tutorials".into(),
                        },
                    },
                    required: vec!["topic".into()],
                },
            })
            .unwrap(),
        ];
        to_string(&list_tools::ListToolsOutput {
            jsonrpc: "2.0".into(),
            result: list_tools::Result { tools },
        })
        .unwrap()
    }

    #[tool(description = "Handle a specific tool")]
    fn handle_tool(&self, Parameters(handle_tool::HandleToolInput { name, arguments }): Parameters<handle_tool::HandleToolInput>) -> String {
        let mut result = String::new();
        match name {
            handle_tool::ToolName::SearchDocumentation => {
                if let Some(query_value) = arguments.get(&handle_tool::ToolArgument::Query) {
                    if query_value.is_empty() {
                        result = "❌ Search query cannot be empty".into();
                    } else {
                        let results = DocumentationFiles::search(&query_value);
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
            },
            handle_tool::ToolName::Tutorial => {
                if let Some(topic_value) = arguments.get(&handle_tool::ToolArgument::Topic) {
                    if topic_value.is_empty() {
                        result = "❌ Tutorial topic cannot be empty".into();
                    } else {
                        result = DocumentationFiles::handle_tutorial_request(topic_value);
                    }
                }
            },
        }

        to_string(
            &handle_tool::HandleToolOutput {
                jsonrpc: "2.0".into(),
                result: handle_tool::Result {
                    text: result
                }
            }
        ).unwrap()
    }
}

#[tool_handler]
impl ServerHandler for Features {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Flutter Flame MCP".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
