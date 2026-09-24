use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router, ServiceExt};
use tokio::io::{stdin, stdout};

// 1. Define the input parameters for your tool
// schemars will automatically generate the JSON Schema that tells the LLM how to use this.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct EchoParams {
    message: String,
}

#[derive(Clone)]
struct MyServer;

// 2. The macro wires this block up as the server handler
#[tool_router(server_handler)]
impl MyServer {
    
    // 3. Register the method as a tool. 
    // The description here is what the LLM will read to understand what the tool does [1].
    #[tool(description = "Echoes a message back to the user")]
    async fn echo(
        &self, 
        Parameters(args): Parameters<EchoParams>
    ) -> Result<String, rmcp::ErrorData> {
        // The tool logic
        Ok(format!("Server successfully received: {}", args.message))
    }
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = (stdin(), stdout());
    let service = MyServer;
    
    let server = service.serve(transport).await?;
    let _quit_reason = server.waiting().await?;
    
    Ok(())
}