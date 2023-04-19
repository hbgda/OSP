use std::{collections::HashMap, sync::Arc};
use async_trait::async_trait;
use once_cell::sync::Lazy;

use crate::core::models::ModelValueType;

use super::{Database, DatabaseFilter, PartialFilter, EntryLocation, FilterValue};

type Volatile = HashMap<String, VolatileTable>;
type VolatileTable = Vec<serde_json::Value>;

static mut TABLES: Lazy<Volatile> = Lazy::new(|| {
    log::debug!("Initializing Volatile tables.");
    Volatile::new()
});

#[derive(Clone)]
pub struct VolatileDb;

impl VolatileDb {
    fn get_tables(&self) -> &mut Volatile {
        unsafe { &mut TABLES }
    }

    fn get_table(&self, table_id: &str) -> Result<&VolatileTable, String> {
        match self.get_tables().get(table_id) {
            Some(table) => Ok(table),
            None => Err(format!("Failed to find table {table_id}"))
        } 
    }

    fn get_table_mut(&self, table_id: &str) -> Result<&mut VolatileTable, String> {
        match self.get_tables().get_mut(table_id) {
            Some(table) => Ok(table),
            None => Err(format!("Failed to find table {table_id}"))
        } 
    }
}

#[async_trait]
impl Database for VolatileDb {

    async fn init(_location: &str) -> Arc<VolatileDb> {
        Arc::new(VolatileDb { })
    }

    // Model does not need to be used in this case as Volatile won't conform to an explicit structure.
    async fn create_table(&self, table_id: &str, _model: &Vec<ModelValueType>) -> Result<(), String> {
        let tables = self.get_tables();
        tables.insert(table_id.to_string(), VolatileTable::new());
        Ok(())
    }

    async fn insert(&self, table_id: &str, data: &serde_json::Value) -> Result<EntryLocation, String> {
        let table = self.get_table_mut(table_id)?;
        table.push(data.clone());
        Ok(table.len())
    }

    async fn update(&self, table_id: &str, filter: DatabaseFilter<FilterValue>, data: &serde_json::Value) -> Result<(), String> {
        let loc = self.find(table_id, filter).await?;
        let table = self.get_table_mut(table_id)?;
        let entry = &mut table[loc];
        for (key, value) in data.as_object().unwrap() {
            entry[key] = value.clone();
        }
        Ok(())
    }

    async fn delete(&self, table_id: &str, filter: DatabaseFilter<FilterValue>) -> Result<(), String> {
        let loc = self.find(table_id, filter).await?;
        let table = self.get_table_mut(table_id)?;
        table.remove(loc);
        Ok(())
    }

    async fn get(&self, table_id: &str,  filter: DatabaseFilter<FilterValue>) -> Result<serde_json::Value, String> {
        let loc = self.find(table_id, filter).await?;
        self.get_loc(table_id, loc).await
    }

    async fn find(&self, table_id: &str, filter: DatabaseFilter<FilterValue>) -> Result<EntryLocation, String> {
        let entries = self.get_table(table_id)?;
        'entry_loop: for (idx, entry) in entries.iter().enumerate() {
            for partial in &filter.0 {
                match partial {
                    PartialFilter::EQ { key, value } => {
                        match entry.get(key) {
                            Some(val) => {
                                if val != value {
                                    continue 'entry_loop
                                }
                            },
                            None => continue 'entry_loop
                        }
                    },
                    PartialFilter::NEQ { key, value } => {
                        match entry.get(key) {
                            Some(val) => {
                                if val == value {
                                    continue 'entry_loop;
                                }
                            },
                            None => continue 'entry_loop
                        }
                    },
                }
            }
            return Ok(idx);
        }
        Err("Failed to find entry matching the filter.".to_string())
    }

    async fn get_loc(&self, table_id: &str, loc: EntryLocation) -> Result<serde_json::Value, String> {
        let table = self.get_table(table_id)?;

        if let Some(entry) = table.get(loc) {
            // return match serde_json::from_value::<T>(entry.clone()) {
            //     Ok(parsed) => Ok(parsed),
            //     Err(e) => {
            //         Err(format!("Failed to parse entry: {}, {entry}", e.to_string()))
            //     }
            // };
            return Ok(entry.clone());
        }

        Err(format!("Failed to find entry at location {loc} in table {table_id}"))
    }
}

#[cfg(test)]
pub mod test {
    use serde::{Deserialize, Serialize};

    use crate::core::database::Database;

    use super::VolatileDb;

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct User {
        email: String,
        password: String
    }

    #[test]
    fn filter() {
        let db = VolatileDb { };
        let filter = db.filter()
            .eq("email".into(), "test@email.com".into())
            .eq("password".into(), "testPassword123".into())
            .build();

        assert_eq!(filter.0.len(), 2);
    }

    #[tokio::test]
    async fn get() {
        let db = VolatileDb { };
        let filter = db.filter()
            .eq("email".into(), "test@email.com".into())
            .eq("password".into(), "testPassword123".into())
            .build();

        let result = db.get("users", filter).await;
        assert_eq!(result.unwrap(), serde_json::json!({ "email": "test@email.com", "password": "testPassword123" }))
    }

}