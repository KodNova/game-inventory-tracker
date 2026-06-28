use core::result::Result::{Err, Ok};

use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
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
        .route("/get_recent_games", get(recent_games_handler))
        .layer(cors)
        .with_state(db);

    axum::serve(listener, app).await.expect("serve fail");
}

#[derive(Deserialize)]
struct Game {
    rawg_id: i32,
    name: String,
    user_token: String,
}

async fn add_game_handler(State(db): State<PgPool>, Json(payload): Json<Game>) -> StatusCode {
    let db_res = add_game_db(db, payload).await;

    match db_res {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// only adds to db nothing else
async fn add_game_db(db: PgPool, game: Game) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO games (rawg_id, name, user_token) VALUES ($1, $2, $3)",
        game.rawg_id,
        game.name,
        game.user_token,
    )
    .execute(&db)
    .await?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct RAWGGame {
    id: u32,
    name: String,
    released: Option<String>,
    background_image: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct GamesResponse {
    results: Vec<RAWGGame>,
}

async fn recent_games_handler() -> Result<Json<GamesResponse>, StatusCode> {
    let api_key = std::env::var("RAWG_API").expect("RAWG_API not set");
    let url = format!(
        "https://api.rawg.io/api/games?key={}&ordering=-released&page_size=20",
        api_key
    );

    let res = reqwest::get(&url)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
        .json::<GamesResponse>()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(Json(res))
}
