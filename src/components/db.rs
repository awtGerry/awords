use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
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

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            match SqliteConnection::connect("sqlite:Awords.db").await {
                Ok(conn) => Ok(conn),
                Err(e) => Err(ServerFnError::ServerError(e.to_string())),
            }
            // Ok(SqliteConnection::connect("sqlite:Awords.db").await?)
        }
    }
}

// Get all users (for testing purposes)
#[server(GetUsers, "/api")]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let req = use_context::<actix_web::HttpRequest>();

    if let Some(req) = req {
        println!("Request: {:#?}", req);
    }
    use futures::TryStreamExt;

    let mut conn = db().await?;
    let mut users = Vec::new();

    let mut rows = sqlx::query_as::<_, User>("SELECT * FROM users").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        users.push(row);
    }

    Ok(users)

}

// Encode with Cbor
#[server(AddUser, "/api", "Cbor")]
pub async fn new_user(email: String, username: String, password: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    // Fake API delay
    std::thread::sleep(std::time::Duration::from_secs(1250));
    match sqlx::query("INSERT INTO users (email, username, password) VALUES (?, ?, ?)")
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
