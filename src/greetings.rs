use std::time::{SystemTime, UNIX_EPOCH};

use axum::{response::IntoResponse, extract::State};
use sqlx::FromRow;

use crate::global_types::DbPool;

#[allow(unused)]
#[derive(Debug, FromRow)]
struct Greetings {
    pub id: i64,
    pub stamp: time::Date,
}

async fn greetings_err(pool: DbPool) -> anyhow::Result<String> {
    let datetime =
        time::OffsetDateTime::UNIX_EPOCH + SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    sqlx::query("INSERT INTO greetings (stamp) VALUES (?)")
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
        .for_each(|it| write!(result, "{it:?}\n").unwrap());

    Ok(format!("{}, data: {}", result.len(), pool.size()))
}

// Just a basic greeting.
pub async fn greetings(State(pool): State<DbPool>) -> impl IntoResponse {
    let value = greetings_err(pool).await;
    match value {
        Err(err) => format!("Errored! {:?}", err),
        Ok(resp) => resp,
    }
}
