use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::{Html, Json},
    routing::get,
};
use cwhelper::lexicon::{Lexicon, indexed::IndexedLexicon, words};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Instant};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    lexicons: HashMap<Language, IndexedLexicon>,
}

#[derive(Deserialize, PartialEq, Eq, Hash)]
enum Language {
    English,
    Dutch,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::Dutch => "Dutch",
            Language::English => "English",
        };
        write!(f, "{s}")
    }
}

#[derive(Deserialize)]
struct WordQuery {
    word: String,
    #[serde(default)]
    language: Language,
}

impl Default for Language {
    fn default() -> Self {
        Self::Dutch
    }
}

#[derive(Serialize)]
struct MatchesResponse {
    matches: Vec<String>,
}

async fn index() -> Html<&'static str> {
    info!("Serving Dutch page");
    Html(include_str!("../../static/index.html"))
}

async fn index_en() -> Html<&'static str> {
    info!("Serving English page");
    Html(include_str!("../../static/index_en.html"))
}

async fn matches(
    State(state): State<Arc<AppState>>,
    Query(params): Query<WordQuery>,
) -> Result<Json<MatchesResponse>, (StatusCode, String)> {
    let start = Instant::now();
    let matches = state.lexicons[&params.language].find_matches(&params.word);
    info!(
        "Extracted {} matche(s) for '{}' from {} in {:?}.",
        matches.len(),
        &params.word,
        &params.language,
        Instant::now() - start
    );
    Ok(Json(MatchesResponse { matches }))
}

fn setup_logging() {
    let file_appender = tracing_appender::rolling::daily("logs", "cwhelper.log");

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(file_appender))
        .with(fmt::layer())
        .with(LevelFilter::INFO)
        .init();
}

fn build_lexicons() -> HashMap<Language, IndexedLexicon> {
    let start = Instant::now();
    let lexicons = HashMap::from([
        (Language::Dutch, IndexedLexicon::from_words(words::dutch())),
        (
            Language::English,
            IndexedLexicon::from_words(words::english()),
        ),
    ]);
    info!("Created lexicons in {:?}.", Instant::now() - start);
    lexicons
}

#[tokio::main]
async fn main() {
    setup_logging();

    let lexicons = build_lexicons();

    let state = Arc::new(AppState { lexicons });

    let app = Router::new()
        .route("/", get(index))
        .route("/en", get(index_en))
        .route("/matches", get(matches))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
