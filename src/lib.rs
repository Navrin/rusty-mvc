//! # Provides an express-like format for rust servers.
//!
//! This is a learning project, expanding on the last chapter of the book.

pub mod server;
pub use server::Server;
pub use server::router;
pub use server::request;
pub use server::response;
