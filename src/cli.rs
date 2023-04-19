#[derive(clap::Parser)]
pub struct CLI {
    #[arg(value_enum, long)]
    /// Type of DB to use.
    pub db: ArgDb,

    #[arg(long)]
    /// Not required for volatile
    pub db_location: Option<String>,

    /// If present, will create the required tables for your database.
    /// Will default to true if Volatile is used.
    #[arg(long, default_value_t = false)]
    pub setup_db: bool,

    /// Generate dummy data for the database.
    #[arg(long, default_value_t = false)]
    pub dummy_db: bool
}

#[derive(clap::ValueEnum, Clone, PartialEq)]
pub enum ArgDb {
    Volatile,
    Persistent,
    Production
}