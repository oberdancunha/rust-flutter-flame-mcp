use rmcp::{
    ServerHandler,
    handler::server::tool::{Parameters, ToolRouter},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};
use serde_json::to_string;

use crate::{
    JSONRPC_VERSION, PROTOCOL_VERSION, SERVER_INFO_INSTRUCTIONS, SERVER_NAME, SERVER_VERSION,
    domain::{
        resource::use_case::{handle_resource::HandleResource, list_resources::ListResources},
        tool::use_case::{
            list_tools::ListTools, search_documentation::SearchDocumentation,
            search_tutorial::SearchTutorial,
        },
    },
    structs::routes::{handle_resource, handle_tool, initialize, list_resources, list_tools},
};

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
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: initialize::Result {
                protocol_version: PROTOCOL_VERSION.to_owned(),
                capabilities: initialize::Capabilities {
                    resources: initialize::Resources { list_changed: true },
                    tools: initialize::Tools { list_changed: true },
                },
            },
            server_info: initialize::ServerInfo {
                name: SERVER_NAME.to_owned(),
                version: SERVER_VERSION.to_owned(),
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
            jsonrpc: JSONRPC_VERSION.to_owned(),
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
        let content = HandleResource::execute(&uri);

        to_string(&handle_resource::HandleResourceOutput {
            jsonrpc: JSONRPC_VERSION.to_owned(),
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
        let tools = ListTools::execute();

        to_string(&list_tools::ListToolsOutput {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: list_tools::Result { tools },
        })
        .unwrap()
    }

    #[tool(description = "Handle a specific tool")]
    fn handle_tool(
        &self,
        Parameters(handle_tool::HandleToolInput { name, arguments }): Parameters<
            handle_tool::HandleToolInput,
        >,
    ) -> String {
        let result = match name {
            handle_tool::ToolName::SearchDocumentation => SearchDocumentation::execute(arguments),
            handle_tool::ToolName::Tutorial => SearchTutorial::execute(arguments),
        };

        to_string(&handle_tool::HandleToolOutput {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: handle_tool::Result { text: result },
        })
        .unwrap()
    }
}

#[tool_handler]
impl ServerHandler for Features {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(SERVER_INFO_INSTRUCTIONS.to_owned()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
