use core::result::Result::{Err, Ok};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
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
        .route("/get_user_games/{user_token}", get(user_games_handler))
        .layer(cors)
        .with_state(db);

    axum::serve(listener, app).await.expect("serve fail");
}

// add games

#[derive(Deserialize, Serialize)]
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

// recent games

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
    let today = chrono::Utc::now().format("%Y-%m-%d");

    let url = format!(
        "https://api.rawg.io/api/games?key={}&ordering=-released&page_size=20&dates=1990-01-01,{}",
        api_key, today
    );

    let res = reqwest::get(&url)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
        .json::<GamesResponse>()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(Json(res))
}

//get users games

#[derive(Serialize)]
struct UserGamesResponse {
    user_games: Vec<Game>,
}

async fn user_games_handler(
    State(db): State<PgPool>,
    axum::extract::Path(user_token): axum::extract::Path<String>,
) -> Result<Json<UserGamesResponse>, StatusCode> {
    let user_games = get_user_games(db, user_token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UserGamesResponse { user_games }))
}

async fn get_user_games(db: PgPool, user_token: String) -> Result<Vec<Game>, sqlx::Error> {
    let games = sqlx::query_as!(
        Game,
        "SELECT rawg_id, name, user_token FROM games WHERE user_token = $1",
        user_token
    )
    .fetch_all(&db)
    .await?;

    Ok(games)
}
