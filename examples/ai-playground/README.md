# Morpheus AI Playground - The Complete Loop

This is the **full Morpheus vision** working end-to-end:

```
User: "Create a counter"
   ↓
AI: Generates Rust code
   ↓
Morpheus: Compiles (might fail)
   ↓
   ├─ Success → Hot-reload into browser ✅
   │  User sees working component
   │
   └─ Error → Feed back to AI 🔄
      AI: Fixes code
      Morpheus: Recompiles
      (Repeats up to 5 times)
      Success → Hot-reload ✅
```

**The app never breaks, even through multiple debugging rounds!**

## What This Proves

✅ **Complete AI Integration** - Natural language → Working component
✅ **Automatic Retry Loop** - AI sees errors and fixes them
✅ **Hot-Reload** - Components load into running app
✅ **Safety Gate** - Compilation failures don't break the app
✅ **The Full Vision** - Everything Morpheus promises, working together

## Quick Start

### 1. Prerequisites

You need:
- Rust toolchain: https://rustup.rs/
- wasm-pack: `cargo install wasm-pack`
- wasm32 target: `rustup target add wasm32-unknown-unknown`
- **Anthropic API key**: https://console.anthropic.com/

### 2. Setup

```bash
cd examples/ai-playground

# Copy environment template
cp .env.example .env

# Edit .env and add your API key:
# ANTHROPIC_API_KEY=sk-ant-your-key-here
```

### 3. Run

```bash
cargo run --bin morpheus-server

# Opens at: http://127.0.0.1:3000
```

### 4. Use

1. Open http://127.0.0.1:3000 in your browser
2. Type what you want: "Create a counter with buttons"
3. Click "Generate Component"
4. Watch the AI create it:
   - Generates Rust code
   - Compiles to WASM (5-10 seconds)
   - If errors: AI sees them and retries
   - If success: Hot-loads into browser
5. See your working component!
6. Try another feature → repeat

## Example Requests

**Simple:**
- "Create a counter with increment and decrement buttons"
- "Build a color picker that changes the background"
- "Make a simple calculator"

**Medium:**
- "Create a todo list where I can add and remove items"
- "Build a random quote generator with a refresh button"
- "Make a stopwatch with start, stop, and reset"

**Advanced:**
- "Create a markdown editor with live preview"
- "Build a tic-tac-toe game"
- "Make a gradient generator with sliders"

## The Complete Loop in Action

### Example Session

```
User: "Create a counter"

Iteration 1:
  AI: Generates Rust code
  Compile: ❌ Failed (missing import)
  → Feed error to AI

Iteration 2:
  AI: Fixed import
  Compile: ❌ Failed (wrong function signature)
  → Feed error to AI

Iteration 3:
  AI: Fixed signature
  Compile: ✅ Success!
  → Hot-reload into browser
  → User sees working counter

User: "Add a reset button"

Iteration 1:
  AI: Adds reset button
  Compile: ✅ Success!
  → Hot-reload (counter still at same value!)
  → User sees reset button
```

**Key point:** App kept running through 2 failed compilations. This is the safety gate!

## Architecture

```
Frontend (Browser)
    ↓ POST /api/generate
    ↓ { prompt: "Create a counter" }
    ↓
Backend (Rust/Axum)
    ↓
Claude API
    ↓ Returns Rust code
    ↓
SubprocessCompiler
    ├─ rustc → WASM
    │
    ├─ Success → Return WASM bytes (base64)
    │
    └─ Error → Feed back to Claude
       Claude → Fixed code
       Compiler → Try again
       (Max 5 iterations)
    ↓
Frontend receives WASM
    ↓
WebAssembly.instantiate()
    ↓
Component renders to DOM
```

## How It Works

### 1. User Request
Frontend sends user's natural language request to backend.

### 2. AI Generation
Backend calls Claude API with:
- System prompt (Rust/WASM code generation rules)
- User request
- Conversation history (for error retry)

### 3. Compilation
Morpheus compiler tries to compile the AI-generated code:
- Uses `SubprocessCompiler` (from Phase 1)
- Runs `wasm-pack build`
- Captures any errors

### 4. Error Handling (The Key Innovation!)
If compilation fails:
```rust
// Add AI's code to conversation
conversation.push(assistant: ai_generated_code)

// Add error for AI to see
conversation.push(user: "Compilation failed with: {error}")

// Call Claude API again
let fixed_code = claude.generate(conversation)

// Retry compilation
compile(fixed_code) // Repeats up to 5 times
```

**This is the recursive failure prevention mechanism!**

### 5. Success & Hot-Reload
When compilation succeeds:
- WASM bytes encoded as base64
- Sent to frontend
- `WebAssembly.instantiate()` runs it
- Component's `#[wasm_bindgen(start)]` executes
- Renders to `#component-root`

### 6. State Preservation (Future)
Currently state is lost on reload. Phase 6 will add:
- Serialize component state before reload
- Load new WASM
- Restore state
- Component continues with same data

## The Safety Mechanism

**JavaScript (recursive failure):**
```javascript
eval(brokenCode) // Runtime error!
// App crashes
// Can't fix it because app is broken
```

**Morpheus (safe retry):**
```rust
match compiler.compile(ai_code) {
    Ok(wasm) => app.hot_reload(wasm),
    Err(error) => {
        // App still running!
        show_user(error)
        ai.fix(error).retry()
    }
}
```

## API Key Security

**Important:** This is a development demo. For production:
- Move API key to backend environment variables ✅ (we do this)
- Don't expose API key to frontend ✅ (we don't)
- Add rate limiting
- Add authentication
- Use server-side sessions

## Limitations & Future Work

**Current Limitations:**
1. **State is lost on reload** - Counter resets to 0
2. **Only one component** - Can't compose multiple
3. **Browser refresh required** - Not true hot-reload yet
4. **Max 5 retry iterations** - Might need more for complex requests
5. **No permission system** - Components can do anything

**Phase 5 Improvements (this is Phase 5):**
- ✅ AI integration working
- ✅ Error retry loop working
- 🔄 State preservation (in progress)
- 🔄 Multi-component composition
- 🔄 WebSocket for no-refresh reload

**Phase 6 Additions:**
- Permission enforcement
- State snapshots & rollback
- Component isolation
- Audit logging
- A/B testing (run v1 and v2 simultaneously)

## Troubleshooting

**"ANTHROPIC_API_KEY not configured"**
- Set it in `.env` file
- Or: `export ANTHROPIC_API_KEY=sk-ant-...`

**"Failed after 5 attempts"**
- Request might be too complex
- Try breaking it into simpler parts
- Check logs to see what errors occurred

**WASM loading fails**
- Check browser console for errors
- Make sure wasm-pack is installed
- Verify wasm32 target is added

**Server won't start**
- Port 3000 might be in use
- Try: `lsof -ti:3000 | xargs kill`
- Or change port in `main.rs`

## Example: Watching It Debug Itself

**Request:** "Create a todo list"

**Iteration 1:**
```rust
// AI generates code but forgets wasm_bindgen import
use web_sys::{Document, Element};

#[wasm_bindgen]  // ❌ ERROR: cannot find attribute `wasm_bindgen` in this scope
pub struct TodoList { ... }
```

**Morpheus:** ❌ Compilation failed
**Action:** Feed error to AI

**Iteration 2:**
```rust
// AI adds missing import
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

#[wasm_bindgen]  // ✅ Now it works
pub struct TodoList { ... }

pub fn add_todo(&mut self, text: &str) {  // ❌ ERROR: must be in impl block
    ...
}
```

**Morpheus:** ❌ Compilation failed
**Action:** Feed error to AI

**Iteration 3:**
```rust
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

#[wasm_bindgen]
pub struct TodoList { todos: Vec<String> }

#[wasm_bindgen]  // ✅ Fixed
impl TodoList {
    pub fn add_todo(&mut self, text: &str) { ... }  // ✅ Fixed
}
```

**Morpheus:** ✅ Compilation successful!
**Action:** Hot-reload into browser
**Result:** Working todo list appears!

**Total time:** ~15-30 seconds
**User interaction:** Typed one sentence
**Debugging rounds:** 2 automatic retries
**App stayed running:** ✅ Never crashed

## This Is The Vision

This playground demonstrates **exactly** what Morpheus is designed to do:

> "Build applications that users can safely modify through natural language conversation with AI agents."

- User speaks naturally ✅
- AI generates code ✅
- Code must compile (safety gate) ✅
- Errors trigger automatic retry ✅
- Success loads into running app ✅
- App never breaks ✅
- Repeat indefinitely ✅

**The recursive failure problem is solved.**

## Next: Try It Yourself

1. Set up your API key
2. Run the server
3. Request something simple: "counter with buttons"
4. Watch it generate, compile, and load
5. Request an addition: "add a reset button"
6. See it update without breaking
7. Request something that might fail: "add rocket ships"
8. Watch it retry and fix itself

**Welcome to self-modifying applications with AI! 🧬**

## Code Structure

```
examples/ai-playground/
├── src/
│   └── main.rs          # Backend server
│       ├── /api/generate endpoint
│       ├── Claude API integration
│       ├── Compilation with retry loop
│       └── Error feedback to AI
├── public/
│   └── index.html       # Frontend UI
│       ├── User input form
│       ├── Component display area
│       ├── Status & logs
│       └── WASM loader
├── .env.example         # API key template
└── README.md           # This file
```

## Contributing

This is the proof-of-concept for Morpheus. Ideas for improvement:

- Better prompt engineering for more reliable code generation
- Streaming responses (show AI thinking in real-time)
- Component library (common patterns)
- State preservation across reloads
- Multi-component composition
- Permission dialogs for user approval
- Rollback to previous versions
- Diff view (show what changed)

## Related Examples

- **Phase 1** (`examples/compiler-test`) - Runtime compilation alone
- **Phase 3** (`examples/integration-test`) - Compiler + runtime integration
- **Phase 4** (`examples/visual-demo`) - Manual hot-reload demo
- **Phase 5** (`examples/ai-playground`) - ⬅️ YOU ARE HERE - The complete AI loop

## License

MIT OR Apache-2.0
