use std::{env, fs::File, io::Read};

use axum::{extract::State, routing::get, Router};
use sonder_backend::{
    global_types::{AppState, DbPool, Secrets},
    greetings::greetings,
};
use sqlx::sqlite::SqlitePoolOptions;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};

// Environment variables defined in .env
const SERVER_BIND_URL: &str = "SERVER_BIND_URL";
const DATABASE_URL: &str = "DATABASE_URL";
const AUTO_LOGOUT_AFTER_DAYS: &str = "AUTO_LOGOUT_AFTER_DAYS";
const SECRETS_FILE: &str = "SECRETS_FILE";

async fn login(State(AppState { secrets: _, .. }): State<AppState>, session: Session) -> String {
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
    dotenvy::dotenv().ok(); // Setup dotenvy to load env vars from .env
    env_logger::init(); // Setup logging

    let app_state = AppState {
        db_pool: setup_and_return_connection_pool().await?,
        secrets: load_secrets(),
    };

    let app = Router::new()
        .route("/greet", get(greetings))
        .route("/login", get(login))
        .route("/logout", get(logout))
        .with_state(app_state)
        .layer(setup_and_return_session_layer());

    let bind_url = env::var(SERVER_BIND_URL).unwrap();

    // run our app with hyper, ensure that SERVER_BIND_URL is set is .env
    let listener = tokio::net::TcpListener::bind(&bind_url).await.unwrap();

    log::info!("Starting server on {}", bind_url);

    axum::serve(listener, app).await?;

    Ok(())
}

/// Return a connection pool that will be setup as a state variable
/// in axum. Server instances can use this pool to connect to the database.
/// See https://sqlx.dev/article/A_Beginners_Guide_to_SQLX_Getting_Started.html
async fn setup_and_return_connection_pool() -> anyhow::Result<DbPool> {
    let database_url = env::var(DATABASE_URL).unwrap();

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .idle_timeout(Some(std::time::Duration::from_secs_f32(5.0)))
        .connect(&database_url)
        .await?;

    Ok(pool)
}

/// Setup sessions using tower_sessions
fn setup_and_return_session_layer() -> SessionManagerLayer<MemoryStore> {
    let autologout = env::var(AUTO_LOGOUT_AFTER_DAYS)
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let store = MemoryStore::default();
    SessionManagerLayer::new(store).with_expiry(tower_sessions::Expiry::OnInactivity(
        time::Duration::days(autologout),
    ))
}

/// Load secrets from SECRETS_FILE (OAuth right now)
fn load_secrets() -> Secrets {
    let path = env::var(SECRETS_FILE).unwrap();
    let toml = {
        let mut toml = String::new();
        let _ = File::open(path).unwrap().read_to_string(&mut toml);
        toml
    };
    toml::from_str(&toml).unwrap()
}
