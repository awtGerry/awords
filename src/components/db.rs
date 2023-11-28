use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    id: i32,
    email: String,
    username: String,
    password: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, SqliteConnection};

        /* pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            Ok(SqliteConnection::connect("sqlite:Awords.db").await.map_err(|e| ServerFnError::ServerError(e.to_string()))?)
        } */
        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            Ok(SqliteConnection::connect("sqlite:Awords.db").await?)
        }
    }
}

// Get all users (for testing purposes)
#[server(GetUsers, "/api")]
pub async fn get_users(email: String, password: String) -> Result<User, ServerFnError> {
    let req = use_context::<actix_web::HttpRequest>();

    if let Some(req) = req {
        println!("Request: {:#?}", req);
    }
    use futures::TryStreamExt;

    let mut conn = db().await?;
    let user: User = sqlx::query_as::<_, User>("
        SELECT * FROM usuarios
        WHERE email = $1 AND password = $2
    ")
    .bind(email)
    .bind(password)
    .fetch_one(&mut conn)
    .await?;

    Ok(user)
}

// Encode with Cbor
#[server(AddUser, "/api", "Cbor")]
pub async fn new_user(email: String, username: String, password: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    // Fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    match sqlx::query("INSERT INTO users (email, username, password) VALUES ($1, $2, $3)")
        .bind(email)
        .bind(username)
        .bind(password)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
