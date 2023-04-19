use serde::{Deserialize, Serialize};

use crate::core::{database::DatabaseModel, models::ModelValueType};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Account {
    pub uid: String,
    pub firstname: String,
    pub surname: String,
    pub email: String,
    pub pass_hash: String
}

impl DatabaseModel for Account {
    fn fields() -> Vec<ModelValueType> {
        vec![
            ModelValueType::String { field: "uid" },
            ModelValueType::String { field: "firstname" },
            ModelValueType::String { field: "surname" },
            ModelValueType::String { field: "email" },
            ModelValueType::String { field: "pass_hash" }
        ]
    }
}

impl Account {
    pub fn partial(firstname: String, surname: String, email: String, pass_hash: String) -> Account {
        Account { 
            uid: String::new(), 
            firstname, 
            surname, 
            email, 
            pass_hash
        }
    }
}