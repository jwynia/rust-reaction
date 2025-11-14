//! Morpheus AI Playground - The Complete Loop
//!
//! This demonstrates the full Morpheus vision:
//! 1. User requests feature via UI
//! 2. AI generates Rust/WASM code
//! 3. Compile (might fail)
//! 4. If errors: AI sees them and retries
//! 5. If success: hot-reload into browser
//! 6. Repeat - app never breaks!

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use morpheus_compiler::{Compiler, SubprocessCompiler};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{error, info, warn};

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    compiler: Arc<SubprocessCompiler>,
    conversation: Arc<Mutex<Vec<Message>>>,
    api_key: String,
}

/// A message in the conversation history
#[derive(Clone, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

/// User request from frontend
#[derive(Deserialize)]
struct GenerateRequest {
    prompt: String,
}

/// Response to frontend with generated WASM
#[derive(Serialize)]
struct GenerateResponse {
    success: bool,
    wasm_base64: Option<String>,
    error: Option<String>,
    iterations: u32,
    logs: Vec<String>,
}

/// Claude API request structure
#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

/// Claude API response structure
#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info,morpheus_compiler=debug")
        .init();

    info!("🧬 Starting Morpheus AI Playground");

    // Load environment variables
    dotenvy::dotenv().ok();
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .unwrap_or_else(|_| {
            warn!("ANTHROPIC_API_KEY not set - AI features will not work!");
            warn!("Set it with: export ANTHROPIC_API_KEY=your-key-here");
            String::new()
        });

    // Check compiler tools
    SubprocessCompiler::check_tools()?;
    info!("✓ Rust compiler and wasm-pack available");

    // Initialize compiler
    let compiler = SubprocessCompiler::new().await?;
    info!("✓ Compiler initialized");

    // Create application state
    let state = AppState {
        compiler: Arc::new(compiler),
        conversation: Arc::new(Mutex::new(Vec::new())),
        api_key,
    };

    // Build router
    let app = Router::new()
        .route("/api/generate", post(generate_component))
        .route("/api/health", get(health_check))
        .nest_service("/", ServeDir::new("examples/ai-playground/public"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let addr = "127.0.0.1:3000";
    info!("🚀 Server running at http://{}", addr);
    info!("   Open http://127.0.0.1:3000 in your browser");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "morpheus-ai-playground"
    }))
}

/// Main endpoint: Generate component from user request
async fn generate_component(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {
    info!("Received request: {}", req.prompt);

    let mut logs = Vec::new();
    logs.push(format!("User request: {}", req.prompt));

    // Check API key
    if state.api_key.is_empty() {
        return Ok(Json(GenerateResponse {
            success: false,
            wasm_base64: None,
            error: Some("ANTHROPIC_API_KEY not configured. Set environment variable to use AI features.".to_string()),
            iterations: 0,
            logs,
        }));
    }

    const MAX_ITERATIONS: u32 = 5;
    let mut iteration = 0;

    // Reset conversation for new request
    let mut conversation = state.conversation.lock().await;
    conversation.clear();

    // Add system prompt
    let system_message = create_system_prompt();
    conversation.push(Message {
        role: "user".to_string(),
        content: system_message,
    });

    // Add user request
    conversation.push(Message {
        role: "user".to_string(),
        content: format!("Create a WASM component: {}", req.prompt),
    });

    drop(conversation); // Release lock

    loop {
        iteration += 1;
        logs.push(format!("\n--- Iteration {} ---", iteration));

        if iteration > MAX_ITERATIONS {
            logs.push("❌ Max iterations reached".to_string());
            return Ok(Json(GenerateResponse {
                success: false,
                wasm_base64: None,
                error: Some("Failed after 5 attempts. The AI couldn't generate working code.".to_string()),
                iterations: iteration - 1,
                logs,
            }));
        }

        // Call Claude API
        logs.push("🤖 Asking AI to generate Rust code...".to_string());
        let rust_code = match call_claude_api(&state).await {
            Ok(code) => {
                logs.push(format!("✓ AI generated {} bytes of Rust code", code.len()));
                code
            }
            Err(e) => {
                error!("Claude API error: {}", e);
                return Ok(Json(GenerateResponse {
                    success: false,
                    wasm_base64: None,
                    error: Some(format!("AI API error: {}", e)),
                    iterations: iteration,
                    logs,
                }));
            }
        };

        // Try to compile
        logs.push("⚙️  Compiling Rust → WASM...".to_string());
        match state.compiler.compile(&rust_code).await {
            Ok(wasm_bytes) => {
                // Success!
                logs.push(format!("✅ Compilation successful! Generated {} bytes of WASM", wasm_bytes.len()));
                logs.push(format!("🎉 Component ready after {} iteration(s)", iteration));

                // Encode WASM as base64 for transmission
                let wasm_base64 = base64_encode(&wasm_bytes);

                return Ok(Json(GenerateResponse {
                    success: true,
                    wasm_base64: Some(wasm_base64),
                    error: None,
                    iterations: iteration,
                    logs,
                }));
            }
            Err(e) => {
                // Compilation failed - feed error back to AI
                let error_msg = e.to_string();
                logs.push(format!("❌ Compilation failed:\n{}", error_msg));
                logs.push("🔄 Feeding error back to AI for retry...".to_string());

                // Add error to conversation
                let mut conversation = state.conversation.lock().await;
                conversation.push(Message {
                    role: "assistant".to_string(),
                    content: rust_code,
                });
                conversation.push(Message {
                    role: "user".to_string(),
                    content: format!(
                        "That code failed to compile with this error:\n\n{}\n\nPlease fix the error and provide the corrected code.",
                        error_msg
                    ),
                });
                drop(conversation);

                // Loop will retry
            }
        }
    }
}

/// Call Claude API to generate Rust code
async fn call_claude_api(state: &AppState) -> Result<String, AppError> {
    let conversation = state.conversation.lock().await;
    let messages = conversation.clone();
    drop(conversation);

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &state.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&ClaudeRequest {
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 4096,
            messages,
        })
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        return Err(AppError::ApiError(format!(
            "Claude API returned {}: {}",
            status, body
        )));
    }

    let claude_response: ClaudeResponse = response.json().await?;

    let text = claude_response
        .content
        .first()
        .map(|block| block.text.clone())
        .ok_or_else(|| AppError::ApiError("No content in response".to_string()))?;

    // Extract Rust code from markdown code blocks
    extract_rust_code(&text)
}

/// Extract Rust code from AI response (handles markdown code blocks)
fn extract_rust_code(text: &str) -> Result<String, AppError> {
    // Look for ```rust code blocks
    if let Some(start) = text.find("```rust") {
        let after_marker = &text[start + 7..];
        if let Some(end) = after_marker.find("```") {
            return Ok(after_marker[..end].trim().to_string());
        }
    }

    // Look for generic ``` code blocks
    if let Some(start) = text.find("```") {
        let after_marker = &text[start + 3..];
        if let Some(end) = after_marker.find("```") {
            return Ok(after_marker[..end].trim().to_string());
        }
    }

    // No code blocks found - return entire text
    Ok(text.trim().to_string())
}

/// Create system prompt for Rust/WASM generation
fn create_system_prompt() -> String {
    r#"You are a Rust expert generating WebAssembly components using wasm-bindgen.

CRITICAL RULES:
1. ONLY output Rust code - no explanations, no markdown except code blocks
2. Use wasm-bindgen for all browser interactions
3. Always include: use wasm_bindgen::prelude::*;
4. Components must have #[wasm_bindgen] on structs and impl blocks
5. Use web_sys for DOM manipulation
6. Keep it simple - no external dependencies beyond wasm-bindgen and web-sys

TEMPLATE TO FOLLOW:

```rust
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Window};

fn window() -> Window {
    web_sys::window().expect("no global window")
}

fn document() -> Document {
    window().document().expect("no document")
}

#[wasm_bindgen]
pub struct YourComponent {
    // state here
}

#[wasm_bindgen]
impl YourComponent {
    #[wasm_bindgen(constructor)]
    pub fn new() -> YourComponent {
        YourComponent { /* init */ }
    }

    pub fn render(&self) {
        let root = document().get_element_by_id("component-root")
            .expect("need #component-root");
        root.set_inner_html(&format!(/* your HTML */));
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"Component loaded!".into());
}
```

When you receive compilation errors, ONLY output the fixed code - no explanations."#.to_string()
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
    Reqwest(reqwest::Error),
    ApiError(String),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Anyhow(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Reqwest(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Anyhow(e) => write!(f, "{}", e),
            AppError::Reqwest(e) => write!(f, "{}", e),
            AppError::ApiError(msg) => write!(f, "{}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Anyhow(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Reqwest(e) => (StatusCode::BAD_GATEWAY, e.to_string()),
            AppError::ApiError(msg) => (StatusCode::BAD_GATEWAY, msg),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
