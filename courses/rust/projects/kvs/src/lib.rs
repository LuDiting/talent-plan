#![deny(missing_docs)]
//! a simple kv store

mod errors;
mod kv;
pub use errors::Result;
pub use kv::KvStore;
