//! Morpheus Complete - The Full System
//!
//! Integrates all 6 phases into one production-ready application:
//! - AI code generation with error retry (Phase 5)
//! - Runtime compilation (Phase 1)
//! - Hot-reload (Phase 2/3)
//! - Visual UI (Phase 4)
//! - State preservation (Phase 6)
//! - Version history & rollback (Phase 6)

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use morpheus_compiler::{Compiler, SubprocessCompiler};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{error, info, warn};

/// Application state
#[derive(Clone)]
struct AppState {
    compiler: Arc<SubprocessCompiler>,
    versions: Arc<Mutex<VersionHistory>>,
    conversation: Arc<Mutex<Vec<Message>>>,
    api_key: String,
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
    ai_generated: bool,
}

impl VersionHistory {
    fn new() -> Self {
        Self {
            versions: Vec::new(),
            current_index: 0,
            current_state: None,
        }
    }

    fn add_version(
        &mut self,
        name: String,
        description: String,
        rust_code: String,
        wasm_bytes: Vec<u8>,
        ai_generated: bool,
    ) -> usize {
        let id = self.versions.len();
        let version = ComponentVersion {
            id,
            name,
            description,
            rust_code,
            wasm_base64: base64_encode(&wasm_bytes),
            created_at: Utc::now(),
            state_snapshot: self.current_state.clone(),
            ai_generated,
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
                ai_generated: v.ai_generated,
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
    ai_generated: bool,
}

/// A message in the AI conversation
#[derive(Clone, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

/// Request to generate component with AI
#[derive(Deserialize)]
struct GenerateRequest {
    prompt: String,
}

/// Response to generation request
#[derive(Serialize)]
struct GenerateResponse {
    success: bool,
    version_id: Option<usize>,
    wasm_base64: Option<String>,
    restored_state: Option<serde_json::Value>,
    error: Option<String>,
    iterations: u32,
    logs: Vec<String>,
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

/// Claude API structures
#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

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

    info!("🧬 Starting Morpheus - Complete System");

    // Load environment variables
    dotenvy::dotenv().ok();
    let api_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| {
        warn!("ANTHROPIC_API_KEY not set - AI features will not work!");
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
        versions: Arc::new(Mutex::new(VersionHistory::new())),
        conversation: Arc::new(Mutex::new(Vec::new())),
        api_key,
    };

    // Build router
    let app = Router::new()
        .route("/api/generate", post(generate_component))
        .route("/api/state", post(update_state))
        .route("/api/rollback", post(rollback))
        .route("/api/history", get(get_history))
        .route("/api/health", get(health_check))
        .nest_service("/", ServeDir::new("examples/morpheus-complete/public"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let addr = "127.0.0.1:3002";
    info!("🚀 Morpheus running at http://{}", addr);
    info!("   The complete system - All 6 phases integrated!");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "morpheus-complete",
        "phases": ["compilation", "hot-reload", "integration", "visual-ui", "ai-loop", "safety"]
    }))
}

/// Generate component with AI (integrates Phase 5 + Phase 6)
async fn generate_component(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {
    info!("AI generation request: {}", req.prompt);

    let mut logs = Vec::new();
    logs.push(format!("🎯 User request: {}", req.prompt));

    // Check API key
    if state.api_key.is_empty() {
        return Ok(Json(GenerateResponse {
            success: false,
            version_id: None,
            wasm_base64: None,
            restored_state: None,
            error: Some("ANTHROPIC_API_KEY not configured".to_string()),
            iterations: 0,
            logs,
        }));
    }

    const MAX_ITERATIONS: u32 = 5;
    let mut iteration = 0;

    // Reset conversation
    let mut conversation = state.conversation.lock().await;
    conversation.clear();
    conversation.push(Message {
        role: "user".to_string(),
        content: create_system_prompt(),
    });
    conversation.push(Message {
        role: "user".to_string(),
        content: format!("Create a WASM component: {}", req.prompt),
    });
    drop(conversation);

    // AI + Compilation retry loop
    loop {
        iteration += 1;
        logs.push(format!("\n━━━ Iteration {} ━━━", iteration));

        if iteration > MAX_ITERATIONS {
            logs.push("❌ Max iterations reached".to_string());
            return Ok(Json(GenerateResponse {
                success: false,
                version_id: None,
                wasm_base64: None,
                restored_state: None,
                error: Some("Failed after 5 attempts".to_string()),
                iterations: iteration - 1,
                logs,
            }));
        }

        // Call AI
        logs.push("🤖 Asking AI to generate Rust code...".to_string());
        let rust_code = match call_claude_api(&state).await {
            Ok(code) => {
                logs.push(format!("✓ AI generated {} bytes of code", code.len()));
                code
            }
            Err(e) => {
                error!("Claude API error: {}", e);
                return Ok(Json(GenerateResponse {
                    success: false,
                    version_id: None,
                    wasm_base64: None,
                    restored_state: None,
                    error: Some(format!("AI API error: {}", e)),
                    iterations: iteration,
                    logs,
                }));
            }
        };

        // Compile
        logs.push("⚙️  Compiling Rust → WASM...".to_string());
        match state.compiler.compile(&rust_code).await {
            Ok(wasm_bytes) => {
                // SUCCESS! Now save with state preservation (Phase 6)
                logs.push(format!(
                    "✅ Compilation successful! {} bytes of WASM",
                    wasm_bytes.len()
                ));
                logs.push(format!("🎉 Component ready after {} iteration(s)", iteration));

                // Get current state for preservation
                let mut history = state.versions.lock().await;
                let restored_state = history.current_state.clone();

                // Add to version history with state preservation
                let version_name = format!("AI Generated: {}", truncate(&req.prompt, 40));
                let version_desc = req.prompt.clone();
                let version_id = history.add_version(
                    version_name,
                    version_desc,
                    rust_code,
                    wasm_bytes.clone(),
                    true, // AI generated
                );

                logs.push(format!("📜 Saved as version {} in history", version_id));
                if restored_state.is_some() {
                    logs.push("🔒 State preserved from previous version!".to_string());
                }

                let wasm_base64 = base64_encode(&wasm_bytes);

                return Ok(Json(GenerateResponse {
                    success: true,
                    version_id: Some(version_id),
                    wasm_base64: Some(wasm_base64),
                    restored_state,
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

                let mut conversation = state.conversation.lock().await;
                conversation.push(Message {
                    role: "assistant".to_string(),
                    content: rust_code,
                });
                conversation.push(Message {
                    role: "user".to_string(),
                    content: format!(
                        "That code failed to compile with this error:\n\n{}\n\nFix it.",
                        error_msg
                    ),
                });
                drop(conversation);

                // Loop continues for retry
            }
        }
    }
}

/// Update component state
async fn update_state(
    State(state): State<AppState>,
    Json(req): Json<UpdateStateRequest>,
) -> Result<Json<UpdateStateResponse>, AppError> {
    let mut history = state.versions.lock().await;
    history.update_state(req.state);
    Ok(Json(UpdateStateResponse { success: true }))
}

/// Rollback to previous version
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
async fn get_history(State(state): State<AppState>) -> Result<Json<HistoryResponse>, AppError> {
    let history = state.versions.lock().await;
    Ok(Json(HistoryResponse {
        versions: history.get_history(),
        current_state: history.current_state.clone(),
    }))
}

/// Call Claude API
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

    extract_rust_code(&text)
}

/// Extract Rust code from AI response
fn extract_rust_code(text: &str) -> Result<String, AppError> {
    if let Some(start) = text.find("```rust") {
        let after_marker = &text[start + 7..];
        if let Some(end) = after_marker.find("```") {
            return Ok(after_marker[..end].trim().to_string());
        }
    }

    if let Some(start) = text.find("```") {
        let after_marker = &text[start + 3..];
        if let Some(end) = after_marker.find("```") {
            return Ok(after_marker[..end].trim().to_string());
        }
    }

    Ok(text.trim().to_string())
}

/// Create system prompt for AI
fn create_system_prompt() -> String {
    r#"You are a Rust expert generating WebAssembly components using Leptos 0.6 framework and Tailwind CSS for styling.

CRITICAL RULES:
1. ONLY output Rust code - no explanations, no markdown formatting
2. Use Leptos 0.6 for reactive components
3. Use Tailwind CSS utility classes for ALL styling - NEVER use inline styles
4. Always include proper imports: use leptos::*; use wasm_bindgen::prelude::*;
5. Components must be annotated with #[component]
6. Export a mount() function for WASM initialization

COMPONENT TEMPLATE:

use leptos::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn YourComponent() -> impl IntoView {
    // State using signals
    let (count, set_count) = create_signal(0);

    // Event handlers
    let increment = move |_| set_count.update(|n| *n += 1);

    // View using Tailwind classes
    view! {
        <div class="p-6 max-w-2xl mx-auto">
            <h1 class="text-4xl font-bold text-gray-900 mb-4">
                {move || count.get()}
            </h1>
            <button
                on:click=increment
                class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
                "Increment"
            </button>
        </div>
    }
}

#[wasm_bindgen]
pub fn mount() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <YourComponent/> })
}

LEPTOS 0.6 SYNTAX:
- No cx parameter (removed in 0.6)
- create_signal() without Scope
- Events: on:click, on:input, on:submit
- Reactive: {move || val.get()} or {val}
- Updates: set_val.set(x) or set_val.update(|n| *n + 1)

TAILWIND PATTERNS:

Buttons:
- Primary: "px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
- Danger: "px-6 py-3 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
- Success: "px-6 py-3 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"

Inputs:
- Text: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"

Cards: "bg-white rounded-lg shadow-md p-6"
Alerts: "bg-blue-50 border-l-4 border-blue-500 p-4 rounded"

Layout:
- Container: "max-w-2xl mx-auto px-4"
- Flex: "flex gap-4 items-center justify-between"
- Grid: "grid grid-cols-3 gap-6"

Typography:
- H1: "text-4xl font-bold text-gray-900"
- Body: "text-base text-gray-600"

EXAMPLES:

Counter:
view! {
    <div class="p-6 max-w-2xl mx-auto">
        <h1 class="text-4xl font-bold text-gray-900 text-center mb-6">
            {move || count.get()}
        </h1>
        <div class="flex gap-3 justify-center">
            <button on:click=dec class="px-6 py-3 bg-red-600 text-white rounded-lg hover:bg-red-700">"Decrement"</button>
            <button on:click=inc class="px-6 py-3 bg-green-600 text-white rounded-lg hover:bg-green-700">"Increment"</button>
        </div>
    </div>
}

Todo List:
let (todos, set_todos) = create_signal(vec!["Item 1".to_string()]);
let (input, set_input) = create_signal(String::new());

view! {
    <div class="p-6 max-w-2xl mx-auto">
        <h1 class="text-4xl font-bold text-gray-900 mb-6">"Todos"</h1>
        <div class="flex gap-3 mb-6">
            <input
                type="text"
                prop:value=move || input.get()
                on:input=move |ev| set_input.set(event_target_value(&ev))
                class="flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
                placeholder="Add todo..."/>
            <button
                on:click=move |_| {
                    if !input.get().is_empty() {
                        set_todos.update(|t| t.push(input.get()));
                        set_input.set(String::new());
                    }
                }
                class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700">"Add"</button>
        </div>
        <ul class="space-y-2">
            <For
                each=move || todos.get().into_iter().enumerate()
                key=|(i, _)| *i
                children=move |(i, todo)| view! {
                    <li class="flex items-center gap-3 p-4 bg-white rounded-lg shadow-md">
                        <span class="flex-1">{todo}</span>
                        <button
                            on:click=move |_| set_todos.update(|t| { t.remove(i); })
                            class="px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600">"Remove"</button>
                    </li>
                }
            />
        </ul>
    </div>
}

ERROR HANDLING:
When you receive errors:
1. Read error carefully
2. Fix the issue
3. Output ONLY corrected code
4. No comments

NEVER use inline styles or raw HTML.
ALWAYS use Tailwind classes and Leptos components."#
        .to_string()
}

/// Truncate string
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}

/// Base64 encode
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
