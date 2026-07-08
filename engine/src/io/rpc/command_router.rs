//! Maps external API requests to Command IDs.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::command::pipeline::{CommandPipeline, PipelineError};

#[derive(Deserialize)]
pub struct CommandRequest {
    pub command_id: String,
    pub params: Value,
}

#[derive(Serialize)]
pub struct CommandResponse {
    pub success: bool,
    pub error: Option<String>,
}

/// Routes an incoming JSON request to the Command Pipeline.
pub fn handle_command_request(
    pipeline: &mut CommandPipeline,
    request_json: &str,
) -> Result<String, String> {
    let request: CommandRequest = serde_json::from_str(request_json).map_err(|e| e.to_string())?;
    
    match pipeline.execute(&request.command_id, request.params) {
        Ok(_) => {
            let resp = CommandResponse { success: true, error: None };
            serde_json::to_string(&resp).map_err(|e| e.to_string())
        }
        Err(e) => {
            let err_msg = match e {
                PipelineError::CommandNotFound(id) => format!("Command not found: {}", id),
                PipelineError::ExecutionError(msg) => format!("Execution failed: {}", msg),
                PipelineError::GraphError(err) => format!("Graph error: {:?}", err),
            };
            let resp = CommandResponse { success: false, error: Some(err_msg) };
            serde_json::to_string(&resp).map_err(|e| e.to_string())
        }
    }
}