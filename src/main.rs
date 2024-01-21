use std::{
    env,
    result::Result::Ok,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Context;
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, Pool, Sqlite};
use time::Duration;

// Environment variables defined in .env
const SERVER_BIND_URL: &str = "SERVER_BIND_URL";
const DATABASE_URL: &str = "DATABASE_URL";

type DbPool = Pool<Sqlite>;

#[derive(Debug, FromRow)]
struct Greetings {
    pub id: i64,
    pub stamp: time::Date,
}

async fn greetings_err(pool: DbPool) -> anyhow::Result<String> {
    let datetime =
        time::OffsetDateTime::UNIX_EPOCH + SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    sqlx::query("INSERT INTO greetings (stamp) VALUES ($1)")
        .bind(datetime.date())
        .execute(&pool)
        .await?;

    let table = sqlx::query_as::<_, Greetings>("SELECT id, stamp FROM greetings")
        .fetch_all(&pool)
        .await?;

    let mut result = String::new();
    use std::fmt::Write;

    table
        .into_iter()
        .for_each(|it| write!(result, "{:?}\n", &it).unwrap());

    Ok(format!("{}, data: {}", result.len(), pool.size()))
}

// Just a basic greeting.
async fn greetings(State(pool): State<DbPool>) -> impl IntoResponse {
    let value = greetings_err(pool).await;
    match value {
        Err(err) => format!("Errored! {:?}", err),
        Ok(resp) => resp,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init(); // Setup logging
    dotenvy::dotenv().ok(); // Setup dotenvy to load env vars from .env

    let app = Router::new()
        .route("/", get(greetings))
        .with_state(setup_and_return_connection_pool().await?);

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
