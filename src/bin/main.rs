use std::{env, result::Result::Ok};

use anyhow::Context;
use axum::{routing::get, Router};
use sonder_backend::{global_types::DbPool, greetings::greetings};
use sqlx::sqlite::SqlitePoolOptions;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};

// Environment variables defined in .env
const SERVER_BIND_URL: &str = "SERVER_BIND_URL";
const DATABASE_URL: &str = "DATABASE_URL";

async fn login(session: Session) -> String {
    let val = session.get("COUNT").await.unwrap().unwrap_or(1);
    session.insert("COUNT", val + 1).await.unwrap();
    format!("Login attempt: {val}")
}

async fn logout(session: Session) -> String {
    session.flush().await.unwrap();
    "OK".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init(); // Setup logging
    dotenvy::dotenv().ok(); // Setup dotenvy to load env vars from .env

    let app = Router::new()
        .route("/greet", get(greetings))
        .route("/login", get(login))
        .route("/logout", get(logout))
        .with_state(setup_and_return_connection_pool().await?)
        .layer(setup_and_return_session_layer());

    // run our app with hyper, ensure that SERVER_BIND_URL is set is .env
    let listener = tokio::net::TcpListener::bind(env::var(SERVER_BIND_URL).unwrap())
        .await
        .unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}

/// Return a connection pool that will be setup as a state variable
/// in axum. Server instances can use this pool to connect to the database.
/// See https://sqlx.dev/article/A_Beginners_Guide_to_SQLX_Getting_Started.html
async fn setup_and_return_connection_pool() -> anyhow::Result<DbPool> {
    let database_url = env::var(DATABASE_URL)
        .with_context(|| format!("{DATABASE_URL} not found in environment"))?;

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .idle_timeout(Some(std::time::Duration::from_secs_f32(2.0)))
        .connect(&database_url)
        .await?;

    Ok(pool)
}

/// Setup sessions using tower_sessions
fn setup_and_return_session_layer() -> SessionManagerLayer<MemoryStore> {
    let store = MemoryStore::default();
    SessionManagerLayer::new(store).with_expiry(tower_sessions::Expiry::OnSessionEnd)
}
