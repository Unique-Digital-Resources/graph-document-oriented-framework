//! External API layer (WebSockets/HTTP mapping to Commands and Queries).

pub mod command_router;
pub mod query_router;
pub mod server;

pub use server::RpcServer;