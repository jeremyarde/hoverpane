use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

// Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedValue {
    id: String,
    value: String,
    timestamp: String,
}

// App state
#[derive(Default)]
struct AppState {
    values: HashMap<String, ScrapedValue>,
}

type SharedState = Arc<Mutex<AppState>>;

// Routes
async fn get_values(
    State(state): State<SharedState>
) -> Json<Vec<ScrapedValue>> {
    let state = state.lock().await;
    let values: Vec<ScrapedValue> = state.values.values().cloned().collect();
    Json(values)
}

async fn update_value(
    State(state): State<SharedState>,
    Json(value): Json<ScrapedValue>,
) -> Json<ScrapedValue> {
    let mut state = state.lock().await;
    state.values.insert(value.id.clone(), value.clone());
    Json(value)
}

#[tokio::main]
async fn main() {
    // Initialize state
    let state = Arc::new(Mutex::new(AppState::default()));

    // Build router
    let app = Router::new()
        .route("/values", get(get_values))
        .route("/values", post(update_value))
        .with_state(state);

    // Run server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("API listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
