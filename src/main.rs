use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, error};

#[derive(Clone)]
struct AppState {
    pinecone_api_key: String,
    pinecone_host: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchRequest {
    query: String,
    top_k: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    id: String,
    score: f32,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResponse {
    results: Vec<SearchResult>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting MCPDHR server...");

    // Load environment variables
    let pinecone_api_key = std::env::var("PINECONE_API_KEY")
        .expect("PINECONE_API_KEY not set");
    let pinecone_host = std::env::var("PINECONE_ASSISTANT_HOST")
        .unwrap_or_else(|_| "https://prod-1-data.ke.pinecone.io".to_string());

    let state = AppState {
        pinecone_api_key,
        pinecone_host,
    };

    let state = Arc::new(state);

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/search", post(search_documents))
        .with_state(state);

    // Get port from environment or use default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind");

    info!("Server listening on 0.0.0.0:{}", port);

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// Search endpoint
async fn search_documents(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SearchRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Search request: {:?}", request);

    // TODO: Implement Pinecone integration
    // This is a placeholder that shows the structure
    
    let results = vec![
        SearchResult {
            id: "doc1".to_string(),
            score: 0.95,
            text: "Documento de exemplo 1".to_string(),
        },
    ];

    Ok(Json(SearchResponse { results }))
}
