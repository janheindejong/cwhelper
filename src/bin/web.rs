use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::{Html, Json},
    routing::get,
};
use cwhelper::Lexicon;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

// Shared state: load the lexicon once at startup
struct AppState {
    lexicon: Lexicon,
}

#[derive(Deserialize)]
struct WordQuery {
    word: String,
}

#[derive(Serialize)]
struct MatchesResponse {
    matches: Vec<String>,
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

async fn find_matches(
    State(state): State<Arc<AppState>>,
    Query(params): Query<WordQuery>,
) -> Result<Json<MatchesResponse>, (StatusCode, String)> {
    let matches = state.lexicon.find_matches(&params.word);
    info!("Found {} matches for query {}", matches.len(), &params.word);
    Ok(Json(MatchesResponse { matches }))
}

fn setup_logging() {
    let file_appender = tracing_appender::rolling::daily("logs", "cwhelper.log");

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(file_appender)) // file (JSON-friendly, structured)
        .with(fmt::layer()) // stdout (human-readable)
        .init();
}

#[tokio::main]
async fn main() {
    setup_logging();

    let lexicon = Lexicon::dutch();
    let state = Arc::new(AppState { lexicon });

    let app = Router::new()
        .route("/", get(index))
        .route("/matches", get(find_matches))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
