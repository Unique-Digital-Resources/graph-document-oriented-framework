//! Maps external API requests to Graph queries.

use serde::{Deserialize, Serialize};

use crate::core::graph::queries::GraphQuery;
use crate::core::node::TypeId;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum QueryRequest {
    #[serde(rename = "find_by_type")]
    FindByType { type_id: String },
    #[serde(rename = "find_by_tag")]
    FindByTag { tag: String },
    #[serde(rename = "children")]
    Children { node_id: String },
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub node_ids: Vec<String>,
}

/// Routes an incoming JSON request to the Graph Query Engine.
pub fn handle_query_request(
    query: &GraphQuery,
    request_json: &str,
) -> Result<String, String> {
    let request: QueryRequest = serde_json::from_str(request_json).map_err(|e| e.to_string())?;
    
    let ids = match request {
        QueryRequest::FindByType { type_id } => {
            query.find_by_type(&TypeId::new(type_id))
        }
        QueryRequest::FindByTag { tag } => {
            query.find_by_tag(&tag)
        }
        QueryRequest::Children { node_id } => {
            let id = uuid::Uuid::parse_str(&node_id).map_err(|e| e.to_string())?;
            query.children(id)
        }
    };

    let resp = QueryResponse {
        node_ids: ids.iter().map(|id| id.to_string()).collect(),
    };

    serde_json::to_string(&resp).map_err(|e| e.to_string())
}