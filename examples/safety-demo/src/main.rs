//! Morpheus Safety Demo - Phase 6
//!
//! Demonstrates advanced safety features:
//! 1. State Preservation - Hot-reload without losing data
//! 2. Version History - Track all component versions
//! 3. Rollback/Undo - Revert to previous versions
//!
//! Example:
//! - Counter at 42
//! - Hot-reload to new visual theme
//! - Counter still at 42!
//! - Don't like it? Rollback - counter still at 42!

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{info, warn};

/// Application state
#[derive(Clone)]
struct AppState {
    versions: Arc<Mutex<VersionHistory>>,
}

/// Version history manager
#[derive(Clone)]
struct VersionHistory {
    versions: Vec<ComponentVersion>,
    current_index: usize,
    current_state: Option<serde_json::Value>,
}

/// A versioned component snapshot
#[derive(Clone, Serialize, Deserialize)]
struct ComponentVersion {
    id: usize,
    name: String,
    description: String,
    rust_code: String,
    wasm_base64: String,
    created_at: DateTime<Utc>,
    state_snapshot: Option<serde_json::Value>,
}

impl VersionHistory {
    fn new() -> Self {
        Self {
            versions: Vec::new(),
            current_index: 0,
            current_state: None,
        }
    }

    fn add_version(&mut self, name: String, description: String, rust_code: String, wasm_bytes: Vec<u8>) -> usize {
        let id = self.versions.len();
        let version = ComponentVersion {
            id,
            name,
            description,
            rust_code,
            wasm_base64: base64_encode(&wasm_bytes),
            created_at: Utc::now(),
            state_snapshot: self.current_state.clone(),
        };

        self.versions.push(version);
        self.current_index = id;
        id
    }

    fn get_current(&self) -> Option<&ComponentVersion> {
        self.versions.get(self.current_index)
    }

    fn rollback_to(&mut self, version_id: usize) -> Option<&ComponentVersion> {
        if version_id < self.versions.len() {
            self.current_index = version_id;
            // Restore state from that version
            if let Some(version) = self.versions.get(version_id) {
                self.current_state = version.state_snapshot.clone();
            }
            self.get_current()
        } else {
            None
        }
    }

    fn update_state(&mut self, state: serde_json::Value) {
        self.current_state = Some(state);
    }

    fn get_history(&self) -> Vec<VersionSummary> {
        self.versions
            .iter()
            .map(|v| VersionSummary {
                id: v.id,
                name: v.name.clone(),
                description: v.description.clone(),
                created_at: v.created_at.to_rfc3339(),
                is_current: v.id == self.current_index,
            })
            .collect()
    }
}

/// Version summary for history display
#[derive(Serialize)]
struct VersionSummary {
    id: usize,
    name: String,
    description: String,
    created_at: String,
    is_current: bool,
}

/// Request to load a new component version
#[derive(Deserialize)]
struct LoadVersionRequest {
    name: String,
    description: String,
    rust_code: String,
}

/// Response with WASM and metadata
#[derive(Serialize)]
struct LoadVersionResponse {
    success: bool,
    version_id: usize,
    wasm_base64: String,
    restored_state: Option<serde_json::Value>,
    error: Option<String>,
}

/// Request to update component state
#[derive(Deserialize)]
struct UpdateStateRequest {
    state: serde_json::Value,
}

/// Response to state update
#[derive(Serialize)]
struct UpdateStateResponse {
    success: bool,
}

/// Request to rollback to a version
#[derive(Deserialize)]
struct RollbackRequest {
    version_id: usize,
}

/// Response to rollback
#[derive(Serialize)]
struct RollbackResponse {
    success: bool,
    version_id: usize,
    wasm_base64: String,
    restored_state: Option<serde_json::Value>,
    error: Option<String>,
}

/// Get version history
#[derive(Serialize)]
struct HistoryResponse {
    versions: Vec<VersionSummary>,
    current_state: Option<serde_json::Value>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("🧬 Starting Morpheus Safety Demo (Phase 6)");

    // Create application state
    let state = AppState {
        versions: Arc::new(Mutex::new(VersionHistory::new())),
    };

    // Build router
    let app = Router::new()
        .route("/api/load", post(load_version))
        .route("/api/state", post(update_state))
        .route("/api/rollback", post(rollback))
        .route("/api/history", get(get_history))
        .route("/api/health", get(health_check))
        .nest_service("/", ServeDir::new("examples/safety-demo/public"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let addr = "127.0.0.1:3001";
    info!("🚀 Server running at http://{}", addr);
    info!("   Open http://127.0.0.1:3001 in your browser");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "morpheus-safety-demo"
    }))
}

/// Load a new component version (compiles Rust code)
async fn load_version(
    State(state): State<AppState>,
    Json(req): Json<LoadVersionRequest>,
) -> Result<Json<LoadVersionResponse>, AppError> {
    info!("Loading new version: {}", req.name);

    // For demo purposes, we'll skip actual compilation and use pre-built examples
    // In a real implementation, this would call SubprocessCompiler
    let wasm_bytes = compile_demo_component(&req.rust_code)?;

    let mut history = state.versions.lock().await;
    let restored_state = history.current_state.clone();
    let version_id = history.add_version(
        req.name,
        req.description,
        req.rust_code,
        wasm_bytes.clone(),
    );

    Ok(Json(LoadVersionResponse {
        success: true,
        version_id,
        wasm_base64: base64_encode(&wasm_bytes),
        restored_state,
        error: None,
    }))
}

/// Update the current component state
async fn update_state(
    State(state): State<AppState>,
    Json(req): Json<UpdateStateRequest>,
) -> Result<Json<UpdateStateResponse>, AppError> {
    let mut history = state.versions.lock().await;
    history.update_state(req.state);

    Ok(Json(UpdateStateResponse { success: true }))
}

/// Rollback to a previous version
async fn rollback(
    State(state): State<AppState>,
    Json(req): Json<RollbackRequest>,
) -> Result<Json<RollbackResponse>, AppError> {
    info!("Rolling back to version {}", req.version_id);

    let mut history = state.versions.lock().await;

    if let Some(version) = history.rollback_to(req.version_id) {
        Ok(Json(RollbackResponse {
            success: true,
            version_id: version.id,
            wasm_base64: version.wasm_base64.clone(),
            restored_state: version.state_snapshot.clone(),
            error: None,
        }))
    } else {
        Ok(Json(RollbackResponse {
            success: false,
            version_id: 0,
            wasm_base64: String::new(),
            restored_state: None,
            error: Some(format!("Version {} not found", req.version_id)),
        }))
    }
}

/// Get version history
async fn get_history(
    State(state): State<AppState>,
) -> Result<Json<HistoryResponse>, AppError> {
    let history = state.versions.lock().await;

    Ok(Json(HistoryResponse {
        versions: history.get_history(),
        current_state: history.current_state.clone(),
    }))
}

/// Demo: Compile component (simplified - would use SubprocessCompiler in real implementation)
fn compile_demo_component(_code: &str) -> Result<Vec<u8>, AppError> {
    // For demo purposes, return a minimal WASM module
    // In real implementation, this would call SubprocessCompiler
    warn!("Using demo compilation - replace with SubprocessCompiler for production");

    // Minimal valid WASM module (magic number + version)
    let wasm = vec![
        0x00, 0x61, 0x73, 0x6D, // magic: \0asm
        0x01, 0x00, 0x00, 0x00, // version: 1
    ];

    Ok(wasm)
}

/// Base64 encode bytes
fn base64_encode(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

/// Custom error type
#[derive(Debug)]
enum AppError {
    Anyhow(anyhow::Error),
    Other(String),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Anyhow(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Anyhow(e) => write!(f, "{}", e),
            AppError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Anyhow(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Other(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
