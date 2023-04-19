pub mod volatile;

use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::{models::{ModelValueType, account::Account, session::Session}, accounts};

pub type FilterValue = serde_json::Value;
pub type EntryLocation = usize;

static mut DATABASE: Option<Arc<dyn Database>> = None;

/// Attempts to initialize database with type `<T>`.
/// 
/// # Arguments
/// * `location` - [`String`] containing a database location that can be interpreted by the given database `<T>`. May not be necessary for some systems. (e.g. Volatile)
/// 
/// # Examples
/// ```
/// use crate::core::{database, volatile::VolatileDb};
/// 
/// match database::init<VolatileDb>("").await {
///     Ok(_) => log::info!("Initialized database!"),
///     Err(_) => log::error!("Failed to initialize database!")
/// };
/// 
pub async fn init<T: Database + 'static>(location: &str) -> Result<(), ()> {
    log::debug!("Initalizing database...");
    unsafe {
        if let None = DATABASE {
            DATABASE = Some(
                T::init(location).await
            );
            return Ok(())
        }
        Err(())
    }
}

/// Returns the current database instance, if present.
/// 
/// # Panics
/// Will panic if database has not yet been initialized.
pub fn get() -> Arc<dyn Database> {
    unsafe {
        if let Some(db) = &DATABASE {
            return db.clone();
        }
        panic!("Database hasn't been initialized!")
    }
}

/// Populate the database instance with required tables.
pub async fn setup() {
    log::debug!("Creating tables...");
    let db = get();
    db.create_table("accounts", &Account::fields()).await.unwrap();
    db.create_table("sessions", &Session::fields()).await.unwrap();
    // add as needed
}

/// Populate the database with dummy data, primarily for testing.
pub async fn dummy() {    
    log::debug!("Creating dummy data...");
    let dummy_acc = Account {
        uid: "".into(),
        firstname: "Real".into(),
        surname: "Person".into(),
        email: "person@email.com".into(),
        pass_hash: "TestPassword123".into(),
    };

    log::debug!("Dummy UID {:?}", accounts::register(dummy_acc, true, true).await.unwrap());
}

// Generic database trait
// This will allow a larger degree of freedom in development, 
// as code does not need to be written exlusively for a single database.
// This is especially beneficial during the development process at it means the entire backend can be developed without locking it into a specific database system
#[async_trait]
pub trait Database: Sync + Send {

    async fn init(location: &str) -> Arc<Self> where Self: Sized;

    async fn create_table(&self, table_id: &str, model: &Vec<ModelValueType>) -> Result<(), String>;
    
    // Ordinarily, functions that alter the state of an object should require a mutable reference,
    // however in this case none of the data is being contained within the structure itself, and so does not require mutability.

    /// Create a new entry in the database.
    async fn insert(&self, table_id: &str, data: &serde_json::Value) -> Result<EntryLocation, String>;

    /// Update an existing entry.
    async fn update(&self, table_id: &str, filter: DatabaseFilter<FilterValue>, data: &serde_json::Value) -> Result<(), String>;

    /// Delete an entry.
    async fn delete(&self, table_id: &str, filter: DatabaseFilter<FilterValue>) -> Result<(), String>;

    /// Get an existing entry, will be parsed to `T`.
    /// Will error if the result can't be parsed to `T` or if the query otherwise fails.
    async fn get(&self, table_id: &str,  filter: DatabaseFilter<FilterValue>) -> Result<serde_json::Value, String>;

    /// Create a new filter.
    fn filter(&self) -> DatabaseFilterBuilder<FilterValue> {
        DatabaseFilterBuilder { filter: DatabaseFilter(Vec::new()) }
    }

    /// Find location of entry matching the given filter
    async fn find(&self, table_id: &str, filter: DatabaseFilter<FilterValue>) -> Result<EntryLocation, String>;

    /// Get entry at the given location.
    async fn get_loc(&self, table_id: &str, loc: EntryLocation) -> Result<serde_json::Value, String>;
}

pub struct DatabaseFilterBuilder<T: Clone> {
    filter: DatabaseFilter<T>
}

impl<T: Clone> DatabaseFilterBuilder<T> {
    pub fn build(&self) -> DatabaseFilter<T> {
        self.filter.clone()
    }

    /// Checks entry\[key\] == value
    pub fn eq(&mut self, key: &'static str, value: T) -> &mut Self {
        self.filter.0.push(
            PartialFilter::EQ { key, value }
        );
        self
    }

    /// Checls entry\[key\] != value
    pub fn neq(&mut self, key: &'static str, value: T) -> &mut Self {
        self.filter.0.push(
            PartialFilter::NEQ { key, value }
        );
        self
    }
}

#[derive(Clone, Debug)]
pub struct DatabaseFilter<T>(Vec<PartialFilter<T>>);

#[derive(Clone, Debug)]
pub enum PartialFilter<T> {
    EQ { key: &'static str, value: T },
    NEQ { key: &'static str, value: T }
}

pub trait DatabaseModel: for<'d> Deserialize<'d> + Serialize + Send + Sync + Sized { 
    fn fields() -> Vec<ModelValueType>;
} 