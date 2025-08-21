use crate::{
    modules::uri_files::UriFiles,
    structs::routes::{handle_resource, initialize, list_resources},
};
use rmcp::{
    ServerHandler,
    handler::server::tool::{Parameters, ToolRouter},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};
use serde_json::json;

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
        json!(initialize::InitializeOutput {
            jsonrpc: 2.0,
            result: initialize::Result {
                protocol_version: "2024-11-05".into(),
                capabilities: initialize::Capabilities {
                    resources: initialize::Resources { list_changed: true },
                    tools: initialize::Tools { list_changed: true }
                },
            },
            server_info: initialize::ServerInfo {
                name: "flutter-flame-mcp".into(),
                version: "1.0.0".into(),
                description: "Flame game engine MCP server with on-demand GitHub documentation"
                    .into(),
            }
        })
        .to_string()
    }

    #[tool(description = "List resources available")]
    fn list_resources(&self) -> String {
        let files: Vec<UriFiles> = UriFiles::build_index().unwrap();
        let mut resources: Vec<String> = vec![];
        for file in files.iter() {
            let file_name = &file.uri.replace("flame://", "").replace("/", " > ");
            resources.push(
                json!(list_resources::Resource {
                    uri: file.uri.clone(),
                    name: format!("Flame: {}", file_name),
                    description: format!("Flame engine documentation: {}", file_name),
                    mime_type: "text/markdown".into(),
                })
                .to_string(),
            );
        }

        json!(list_resources::ListResourcesOutput {
            jsonrpc: 2.0,
            result: list_resources::Resources { resources }
        })
        .to_string()
    }

    #[tool(description = "Handle a specific resource")]
    fn handle_resource(
        &self,
        Parameters(handle_resource::HandleResourceInput { uri }): Parameters<
            handle_resource::HandleResourceInput,
        >,
    ) -> String {
        let content = UriFiles::get_content(&uri).unwrap();
        let content = content
            .replace("\r\n", "\n")
            .replace("\r", "\n")
            .replace("\t", "    ");
        let re = regex::Regex::new(r"[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]").unwrap();
        let content = re.replace_all(&content, "").to_string();
        json!(handle_resource::HandleResourceOutput {
            jsonrpc: 2.0,
            result: handle_resource::Result {
                contents: handle_resource::Contents {
                    uri,
                    mime_type: "text/markdown".into(),
                    text: content,
                }
            }
        })
        .to_string()
    }

    #[tool(description = "List tools available")]
    fn list_tools(&self) -> String {
        let tools = json!([
            {
                "name": "search_documentation",
                "description": "Search through Flame documentation",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "Search query"}
                    },
                    "required": ["query"]
                }
            },
            {
                "name": "tutorial",
                "description": "Get complete Flame tutorials with step-by-step instructions for building games (space shooter, platformer, klondike). Use this for learning how to build specific games.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "topic": {
                            "type": "string",
                            "description": "Tutorial topic: \"space shooter\" for complete space shooter game tutorial, \"platformer\" for platformer game tutorial, \"klondike\" for card game tutorial, or \"list\" to see all available tutorials"
                        }
                    },
                    "required": ["topic"]
                }
            },
        ]).to_string();

        json!({
            "jsonrpc": "2.0",
            "result": {"tools": tools}
        })
        .to_string()
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
