use serde::{Deserialize, Serialize};

use crate::core::{database::DatabaseModel, models::ModelValueType};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Session {
    pub session_id: String,
    pub uid: String,
    pub created: i64,
    pub valid: bool
}

impl DatabaseModel for Session {
    fn fields() -> Vec<ModelValueType> {
        vec![
            ModelValueType::String  { field: "session_id" },
            ModelValueType::String  { field: "uid" },
            ModelValueType::Number  { field: "created" },
            ModelValueType::Boolean { field: "valid" }
        ]
    }
}