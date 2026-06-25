use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, response},
    routing::{get, post},
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db = PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("db connect failed");

    //WARN change CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("listener fail");

    let app = Router::new()
        .route("/add_game", post(add_game_handler))
        .layer(cors)
        .with_state(db);

    axum::serve(listener, app).await.expect("serve fail");
}

struct Game {
    rawg_id: i32,
    name: String,
    user_token: String,
}

async fn add_game_handler(State(db): State<PgPool>) -> StatusCode {
    let test_game = Game {
        rawg_id: 1337,
        name: "leet".to_string(),
        user_token: "test".to_string(),
    };
    sqlx::query!(
        "INSERT INTO games (rawg_id, name, user_token) VALUES ($1, $2, $3)",
        test_game.rawg_id,
        test_game.name,
        test_game.user_token,
    )
    .execute(&db)
    .await
    .expect("fails");

    StatusCode::OK
}
