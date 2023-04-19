pub mod cli;
pub mod core;
pub mod middleware;
pub mod routes;

use clap::Parser;
use log::LevelFilter;
use std::sync::Arc;
use std::sync::Mutex;
use tide::sessions::MemoryStore;
use tide::sessions::SessionMiddleware;

use crate::cli::CLI;
use crate::core::database;
use crate::core::database::volatile::VolatileDb;
use crate::core::logger::{Logger, LoggerOptions};
use crate::core::state::ApplicationState;
use crate::middleware::logging::LoggingMiddleware;
use crate::middleware::user_session::UserSessionMiddleware;
use crate::routes::*;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Initialize custom logger
    Logger::new()
        .options(LoggerOptions {
            use_level_colour: true,
            prefix_level: true,
            prefix_datetime: true,
        })
        .max_level(LevelFilter::Debug)
        .set()
        .expect("Failed to initialize logger!");

    let args = CLI::parse();

    // Initialize database depending on the db type passed.
    match args.db {
        cli::ArgDb::Volatile => database::init::<VolatileDb>("").await.unwrap(),
        cli::ArgDb::Persistent => todo!(),
        cli::ArgDb::Production => todo!(),
    };

    // Create necessary tables
    if args.setup_db || args.db == cli::ArgDb::Volatile {
        database::setup().await;
    }

    // Create dummy data
    if args.dummy_db {
        database::dummy().await;
    }

    let mut app = tide::with_state(ApplicationState {
        hb: Arc::new(Mutex::new(handlebars::Handlebars::new())),
    });

    // Setup middleware
    app.with(LoggingMiddleware::new());
    app.with(SessionMiddleware::new(
        MemoryStore::new(),
        b"temp_session_secret_123123412312312", //std::env::var("SESSION_SECRET").unwrap().as_bytes()
    ));
    app.with(UserSessionMiddleware::new());

    // Setup API
    log::info!("| Registering API...");
    api::v1::ApiV1::register(&mut app);

    // Setup site templates
    log::info!("| Registering view templates...");
    client::partials::Partials::register(&mut app);
    client::views::Views::register(&mut app);

    // Serve static content at host/static/
    app.at("/static").serve_dir("static/")?;

    // Start server
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
