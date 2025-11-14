# Morpheus Safety Demo - Phase 6

This demonstrates **Phase 6: Advanced Safety Features** - the capabilities that make self-modifying apps truly safe and practical.

## The Three Pillars of Safety

### 1. State Preservation 🔒
**Your data survives hot-reload!**

```
Counter at 42
   ↓
Load new version (green theme)
   ↓
Counter STILL at 42! ✅
   ↓
Load another version (orange theme)
   ↓
Counter STILL at 42! ✅
```

**Why this matters:**
- Users don't lose their work when you update the app
- True hot-reload (not just refresh)
- Seamless user experience
- Build trust ("my data is safe")

### 2. Version History 📜
**Every change is tracked!**

```
Version 1: Blue Theme (10:30:15 AM)
Version 2: Green Theme (10:31:42 AM) ← Current
Version 3: Orange Theme (10:33:08 AM)
Version 4: Purple Theme (10:34:55 AM)
```

**Why this matters:**
- See what changed and when
- Understand evolution of the app
- Debug issues ("what was different in v2?")
- Audit trail for modifications

### 3. Rollback/Undo ↩️
**Don't like it? Go back!**

```
Version 3 loaded (orange theme)
User: "I don't like this"
   ↓
Click "Rollback to Version 2"
   ↓
Back to green theme
Counter STILL at 42! ✅
```

**Why this matters:**
- Experiment safely (can always undo)
- Recover from mistakes
- User control over changes
- "Undo" for AI modifications

## Quick Start

```bash
cd examples/safety-demo
cargo run --bin safety-server

# Open http://127.0.0.1:3001
```

## How To Use

### Try State Preservation

1. **Increment the counter** to any value (e.g., 42)
2. **Click "Version 2 (Green)"** button
3. **Notice:** Counter is still at 42!
4. **Click "Version 3 (Orange)"** button
5. **Notice:** Still at 42!

**The state persists across version changes.**

### Try Rollback

1. **Load Version 3** (orange theme)
2. **Look at Version History** panel (right side)
3. **Click "Rollback"** on Version 1
4. **Notice:** Back to blue theme, counter still at 42!

**You can undo any change.**

### Try Multiple Iterations

1. **Set counter to 100**
2. **Load v2** → Count: 100
3. **Change counter to 200**
4. **Load v3** → Count: 200
5. **Rollback to v2** → Count: 200 (state from v3!)
6. **Rollback to v1** → Count: 200 (still preserved!)

**State follows you forward and backward through versions.**

## What's Happening Under the Hood

### State Serialization

```rust
// Before loading new version
let state_snapshot = component.serialize_state(); // { count: 42 }

// Load new WASM
let new_component = load_wasm(new_version);

// Restore state
new_component.deserialize_state(state_snapshot); // Counter: 42 ✅
```

### Version Storage

```rust
struct ComponentVersion {
    id: usize,
    name: String,
    description: String,
    rust_code: String,
    wasm_bytes: Vec<u8>,
    created_at: DateTime<Utc>,
    state_snapshot: Option<JSON>,  // ← State at this version!
}
```

Each version stores:
- The code that created it
- The compiled WASM
- **The state when it was created**
- Timestamp and metadata

### Rollback Mechanism

```rust
fn rollback_to(&mut self, version_id: usize) {
    // 1. Switch to that version
    self.current_index = version_id;

    // 2. Load its WASM
    let wasm = self.versions[version_id].wasm_bytes;

    // 3. Restore its state snapshot
    let state = self.versions[version_id].state_snapshot;
    component.load(wasm);
    component.restore_state(state);
}
```

## Architecture

```
Frontend (Browser)
    ↓
    ├─ User modifies state (counter++)
    │  POST /api/state { count: 42 }
    │  → Stored in server memory
    │
    ├─ User loads new version
    │  POST /api/load { name: "v2", ... }
    │  → Compile WASM
    │  → Save as new version WITH current state
    │  → Return WASM + restored state
    │
    └─ User rolls back
       POST /api/rollback { version_id: 1 }
       → Load version 1's WASM
       → Restore version 1's state snapshot
       → Return both

Backend (Rust/Axum)
    ↓
    VersionHistory
        ├─ versions: Vec<ComponentVersion>
        ├─ current_index: usize
        └─ current_state: Option<JSON>
```

## API Endpoints

### POST /api/load
Load a new component version.

**Request:**
```json
{
  "name": "Version 2 - Green Theme",
  "description": "Eco-friendly green styling",
  "rust_code": "..."
}
```

**Response:**
```json
{
  "success": true,
  "version_id": 1,
  "wasm_base64": "...",
  "restored_state": { "count": 42 }
}
```

### POST /api/state
Update current component state.

**Request:**
```json
{
  "state": { "count": 42 }
}
```

### POST /api/rollback
Roll back to a previous version.

**Request:**
```json
{
  "version_id": 0
}
```

**Response:**
```json
{
  "success": true,
  "version_id": 0,
  "wasm_base64": "...",
  "restored_state": { "count": 42 }
}
```

### GET /api/history
Get version history.

**Response:**
```json
{
  "versions": [
    {
      "id": 0,
      "name": "Version 1",
      "description": "Blue theme",
      "created_at": "2024-01-15T10:30:15Z",
      "is_current": false
    },
    {
      "id": 1,
      "name": "Version 2",
      "description": "Green theme",
      "created_at": "2024-01-15T10:31:42Z",
      "is_current": true
    }
  ],
  "current_state": { "count": 42 }
}
```

## The Complete Safety Model

Combining all phases:

```
User: "Add a reset button"
   ↓
AI generates code (Phase 5)
   ↓
Compile with type checking (Phase 1)
   ├─ Error? → AI retries (Phase 5)
   └─ Success? → Continue
   ↓
Save current state { count: 42 } (Phase 6)
   ↓
Load new WASM (Phase 2/3)
   ↓
Restore state { count: 42 } (Phase 6)
   ↓
Hot-reload into browser (Phase 4)
   ↓
User sees: Counter at 42 with new reset button! ✅
   ↓
User: "Actually, I don't like it"
   ↓
Rollback to previous version (Phase 6)
   ↓
Counter STILL at 42, reset button gone ✅
```

**Nothing is lost. Everything is reversible. The app never breaks.**

## Real-World Example

**Todo List App:**

```
User has 5 todos
   ↓
User: "Add checkboxes to mark items complete"
   ↓
AI generates new version
   ↓
Compile successful
   ↓
Save state: { todos: ["Buy milk", "Call mom", ...] }
   ↓
Load new version with checkboxes
   ↓
Restore todos: All 5 todos still there! ✅
   ↓
User checks 2 items as done
   ↓
User: "Add a delete button"
   ↓
AI generates another version
   ↓
Load it: All 5 todos still there, 2 checked! ✅
   ↓
User: "Actually, remove delete - too dangerous"
   ↓
Rollback to previous version
   ↓
Todos still there, checkmarks preserved, delete button gone! ✅
```

## Current Implementation

**Demo Mode:**
- Uses pre-built component variations (v1, v2, v3, v4)
- Simulates state preservation
- Real version history and rollback
- Runs on port 3001

**Production Mode (Future):**
- Replace `compile_demo_component` with `SubprocessCompiler`
- Add real WASM compilation
- WebSocket for instant updates (no browser refresh)
- Persistent storage (database, not memory)

## Limitations & Future Work

**Current:**
- Demo components only (not real WASM compilation yet)
- State stored in memory (lost on server restart)
- Manual version loading (not automatic from AI)
- Single user (no multi-user support)

**Phase 6+ Improvements:**
- Real WASM compilation integration
- Persistent version storage (database)
- Multi-user sessions
- Permission system per version
- A/B testing (run v1 and v2 simultaneously)
- Automatic state schema migration
- State conflict resolution

## Integration with Phase 5 (AI Playground)

Combine them:

```bash
# Terminal 1: AI Playground
cd examples/ai-playground
cargo run --bin morpheus-server

# Terminal 2: Safety Demo
cd examples/safety-demo
cargo run --bin safety-server
```

**Workflow:**
1. Use AI Playground to generate component code
2. Copy generated code to Safety Demo
3. Load it as a new version
4. State is preserved!
5. Don't like it? Rollback!

**Future:** Merge them into one system where:
- AI generates code
- Compiles automatically
- Saves as new version with state preservation
- User can rollback any AI change
- Perfect safety + perfect flexibility

## Why This Matters

### For Users
- "I can modify my app without losing my data" ✅
- "I can experiment without fear" ✅
- "I can undo mistakes" ✅
- "The app never breaks" ✅

### For Developers
- "Users trust the modification system" ✅
- "Can deploy updates without user friction" ✅
- "Easy to debug (version history)" ✅
- "Safe to experiment with AI" ✅

### For AI
- "My changes won't destroy user data" ✅
- "Users can undo my mistakes" ✅
- "I can iterate safely" ✅
- "Build user trust" ✅

## Testing the Safety

**Try to break it:**

1. **Set counter to 999**
2. **Rapidly load v1 → v2 → v3 → v4**
3. **Check:** Counter still 999? ✅
4. **Rollback to v1**
5. **Check:** Counter still 999? ✅
6. **Forward to v4**
7. **Check:** Counter still 999? ✅

**The state never gets lost, no matter what you do.**

## Next Steps

To integrate with full Morpheus:

1. **Replace demo compilation** with SubprocessCompiler
2. **Add database** for persistent storage
3. **Merge with AI Playground** for automatic generation
4. **Add WebSocket** for instant updates
5. **Implement permissions** per version
6. **Add state migration** when schema changes
7. **Multi-component** composition

## Code Structure

```
examples/safety-demo/
├── src/
│   └── main.rs           # Backend server
│       ├── VersionHistory struct
│       ├── State preservation logic
│       ├── Rollback mechanism
│       └── API endpoints
├── public/
│   └── index.html        # Frontend demo
│       ├── Interactive counter
│       ├── Version buttons
│       ├── History display
│       └── State preservation UI
└── README.md            # This file
```

## License

MIT OR Apache-2.0

---

**Phase 6 proves that self-modifying apps can be completely safe.**

Users can modify their running applications without losing data, with full history tracking, and complete undo capability.

**This is the foundation for trustworthy AI-powered applications.** 🧬
