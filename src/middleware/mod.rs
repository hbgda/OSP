use std::ops::{Deref, DerefMut};

use serde_json::json;

pub mod logging;
pub mod user_session;

#[derive(Clone)]
pub struct MiddlewareData(serde_json::Value);

// Deref implementations so the inner json data can be accessed without indexing 0.
impl Deref for MiddlewareData {
    type Target = serde_json::Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MiddlewareData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl MiddlewareData {
    pub fn new() -> MiddlewareData {
        MiddlewareData(json!({}))
    }
}