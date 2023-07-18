use axum::{
    extract::{self, Path},
    http::StatusCode,
    routing::{delete, get, post},
    Extension, Json, Router,
};

use dotenvy::dotenv;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap_or_else(|_| panic!("Failed to create Postgres connection pool! URL: {}", url));

    sqlx::migrate!("./migrations").run(&pool).await?;

    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app().layer(Extension(pool)).into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

/// Having a function that produces our app makes it easy to call it from tests
/// without having to create an HTTP server.
#[allow(dead_code)]
fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/user", post(create_user))
        .route("/users", get(get_users))
        .route("/user/:id", delete(delete_user))
}

async fn handler() -> &'static str {
    "Let's Get Rusty!"
}

async fn get_users(state: Extension<Pool<Postgres>>) -> Json<Vec<User>> {
    let Extension(pool) = state;

    let records = sqlx::query!("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .expect("failed to fetch users");

    let records = records
        .iter()
        .map(|r| User {
            id: Some(r.id),
            name: r.name.to_string(),
            email: r.email.clone(),
        })
        .collect();

    Json(records)
}

pub async fn create_user(
    state: Extension<Pool<Postgres>>,
    extract::Json(user): extract::Json<User>,
) -> Json<User> {
    let Extension(pool) = state;

    let row = sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        user.name,
        user.email
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to create user");

    Json(User {
        id: Some(row.id),
        name: row.name,
        email: row.email,
    })
}

pub async fn delete_user(state: Extension<Pool<Postgres>>, Path(user_id): Path<i32>) -> StatusCode {
    let Extension(pool) = state;

    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&pool)
        .await
        .expect("Failed to delete user");

    StatusCode::NO_CONTENT
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::util::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn hello_world() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Let's Get Rusty!");
    }
}
