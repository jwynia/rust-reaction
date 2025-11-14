# Morpheus - Self-Modifying Applications with AI

**Build applications that users can safely modify through natural language conversation with AI agents.**

## The Problem

You're building an app. A user says: *"Add a dark mode toggle to settings."*

**In TypeScript/JavaScript:**
```typescript
// AI generates code, eval() runs it
eval(aiGeneratedCode);  // 🔥 What could go wrong?

// Everything:
// - Type errors at runtime (app crashes)
// - Can corrupt global state
// - No sandboxing (security risk)
// - Breaks the modification tool itself
// - No way to undo
```

**You've experienced this:** CLI agents with MCP servers where a syntax error means the agent won't load, so you can't even use it to fix the problem. Recursive failure.

**In Rust with Morpheus:**
```rust
// AI generates Rust code
let modification = ai.generate("Add dark mode toggle");

// Framework compiles it BEFORE running
match compiler.compile(&modification) {
    Ok(wasm) => {
        // Type-checked! Safe to load.
        app.hot_reload(wasm);
    }
    Err(errors) => {
        // App still works! Show errors, AI can fix them.
        show_user("AI made a mistake: {}", errors);
        ai.fix_errors(&modification, errors);
    }
}
```

## Why Rust?

Rust provides **5 critical safety mechanisms** TypeScript can't:

### 1. **Compilation as Safety Gate** 🛡️
AI-generated code must type-check before it can run. No runtime type surprises.

### 2. **Ownership = Isolation** 🔒
Components own their state. AI-generated code can't accidentally corrupt other components or global state.

### 3. **WASM Sandboxing** 📦
AI-generated components run in isolated WASM modules with restricted permissions. Can't steal data or attack the system.

### 4. **Atomic Rollback** ⏮️
All modifications are transactional. If something breaks, undo instantly and atomically.

### 5. **Type-Safe Hot Reload** 🔄
New component versions must satisfy interface contracts. Can't break existing functionality.

## User Experience

```
User: "Add a dark mode toggle to settings"
  ↓
AI: Generates Rust code for DarkModeToggle component
  ↓
Morpheus: Compiles to WASM (type-checks!)
  ↓
Morpheus: Shows preview in sandbox
  ↓
User: "Make the toggle bigger"
  ↓
AI: Modifies the component
  ↓
Morpheus: Re-compiles, updates preview
  ↓
User: "Perfect! Apply it"
  ↓
Morpheus: Hot-reloads into live app
  ↓
[User continues using app with new feature]
  ↓
User: "Actually, undo that"
  ↓
Morpheus: Rolls back atomically (instant!)
```

## Architecture

```rust
pub struct MorpheusApp {
    // Dynamic components loaded as WASM modules
    components: ComponentRegistry,

    // Rust compiler (validates AI code)
    compiler: RustToWasmCompiler,

    // AI agent integration
    ai: LLMAgent,

    // Versioned state for rollback
    state: VersionedState,

    // Type system enforces compatibility
    types: TypeRegistry,
}

impl MorpheusApp {
    pub async fn modify(&mut self, user_request: &str) -> Result<()> {
        // 1. AI generates Rust code
        let code = self.ai.generate(user_request).await?;

        // 2. Compile to WASM (type-check!)
        let wasm = self.compiler.compile(&code)?;

        // 3. Take snapshot for rollback
        let snapshot = self.state.snapshot();

        // 4. Hot-reload safely
        match self.hot_reload(wasm) {
            Ok(_) => self.state.commit(snapshot),
            Err(e) => {
                self.state.rollback(snapshot);
                Err(e)
            }
        }
    }
}
```

## Project Status

**Current Phase:** Phase 5 Complete ✅ - THE FULL VISION WORKS!

**What's Working:**
- ✅ **Phase 1:** Runtime Rust compilation (5-10 sec compile times)
- ✅ **Phase 2:** WASM component loading and hot-reload
- ✅ **Phase 3:** Full compiler + runtime integration
- ✅ **Phase 4:** Visual UI component with hot-reload demo
- ✅ **Phase 5:** AI integration - THE COMPLETE LOOP!

**Working Examples:**
- `examples/compiler-test/` - Phase 1: Runtime compilation
- `examples/integration-test/` - Phase 3: Complete compile → load → hot-reload flow
- `examples/visual-demo/` - Phase 4: Interactive counter in browser with 3 visual versions
- `examples/ai-playground/` - **Phase 5: User → AI → Code → Compile → Retry → Success** 🎯

**The Main Loop Works:**
```
User: "Create a counter"
  → AI generates Rust code
  → Compile (may fail)
  → If error: AI sees it and retries (up to 5x)
  → If success: Hot-reload into browser
  → User sees working component!
  → Repeat - app never breaks!
```

**Next Up:**
- **Phase 6:** Advanced safety (permissions, state preservation, rollback)

**Not production-ready yet.** This is research and development proving the concept works.

## Repository Structure

```
rust-reaction/
├── crates/
│   ├── morpheus-core/         # Core types: DynamicComponent, Permissions, State
│   ├── morpheus-compiler/     # Runtime Rust→WASM compilation (Phase 1)
│   └── morpheus-runtime/      # Component loading & hot-reload (Phase 2)
├── examples/
│   ├── compiler-test/         # Phase 1: Runtime compilation
│   ├── integration-test/      # Phase 3: Full integration
│   ├── visual-demo/           # Phase 4: Interactive UI in browser
│   │   ├── src/lib.rs         # Counter component (V1 blue theme)
│   │   ├── public/index.html  # Web page
│   │   └── versions/          # V2 (green) and V3 (animated)
│   └── ai-playground/         # Phase 5: THE COMPLETE LOOP! 🎯
│       ├── src/main.rs        # Backend server with AI + compiler
│       ├── public/index.html  # Frontend UI
│       ├── .env.example       # API key configuration
│       └── README.md          # Full documentation
├── context-network/           # Research, decisions, analysis
│   ├── research/              # Analysis of existing frameworks
│   ├── decisions/             # Key architectural decisions
│   ├── elements/use-cases/    # Self-modifying apps analysis
│   └── planning/              # Implementation strategy & roadmap
└── archive/
    └── prototype-v1/          # Initial exploration (static UI patterns)
```

## Why This is Different from Yew/Leptos/Dioxus

**Existing frameworks answer:** "How do I build web apps in Rust?"

**Morpheus answers:** "How do I build apps that safely modify themselves with AI?"

| Aspect | Traditional Frameworks | Morpheus |
|--------|----------------------|----------|
| **Use Case** | Static app development | Runtime AI modification |
| **Compilation** | Once, at build time | Continuously, at runtime |
| **Code Source** | Human developers | AI agents + humans |
| **Safety** | Compile-time only | Compile + runtime verification |
| **Components** | Bundled, fixed | Dynamic WASM modules |
| **Rollback** | Git/deploy | Real-time undo in app |
| **Sandboxing** | Not needed | Essential |
| **Type Checking** | Development time | Generation time |

**This is complementary, not competitive.** Morpheus could even build on Yew/Leptos for base UI primitives.

## The Real-World Problem This Solves

**You've hit this:**
- CLI coding agents with MCP servers
- Syntax error in plugin
- Agent won't load
- Can't use the agent to fix it
- Stuck!

**Morpheus solves the recursive failure:**
```rust
// Plugin modification fails to compile
match compile_plugin(&ai_plugin) {
    Err(errors) => {
        // Agent STILL WORKS!
        // Can show errors and try again
        show_errors(errors);
        ai.fix_and_retry(errors);
    }
}
```

The tool for modifying can't break itself.

## Documentation

All research, decisions, and analysis are in `context-network/`:

- **[Self-Modifying Apps Use Case](context-network/elements/use-cases/self-modifying-apps.md)** - Detailed problem analysis
- **[Strategic Pivot Decision](context-network/decisions/002-self-modifying-apps-pivot.md)** - Why we're building this
- **[Framework Differentiation](context-network/planning/framework-differentiation-analysis.md)** - How this differs from existing work
- **[Critical Evaluation](context-network/planning/critical-evaluation.md)** - Honest assessment of challenges

## Getting Started

### 🚀 Try The Complete Vision (Phase 5)

**AI Playground - The Full Loop:**

```bash
cd examples/ai-playground

# 1. Setup (copy and edit with your Anthropic API key)
cp .env.example .env
nano .env  # Add: ANTHROPIC_API_KEY=sk-ant-your-key

# 2. Run
cargo run --bin morpheus-server

# 3. Open http://127.0.0.1:3000
# 4. Type: "Create a counter with buttons"
# 5. Watch: AI → Code → Compile → Retry on errors → Success!
```

**What you'll see:**
- User types natural language request
- AI generates Rust/WASM code
- Automatic compilation with error retry (up to 5 times)
- Hot-reload into browser when successful
- Working component appears in ~10-20 seconds
- App never breaks, even through debugging iterations

**See full guide:** `examples/ai-playground/QUICKSTART.md`

---

### 📚 Try Individual Phases

**Phase 1 - Runtime Compilation:**
```bash
cargo run --bin test-compiler
```
Shows Rust code being compiled to WASM at runtime, with error handling.

**Phase 3 - Full Integration:**
```bash
cargo run --bin test-integration
```
Shows the complete flow: compile → load → hot-reload → error handling.

**Phase 4 - Visual Hot-Reload:**
```bash
cd examples/visual-demo
wasm-pack build --target web
python -m http.server 8080
# Visit: http://localhost:8080/public/
```
Interactive counter with 3 visual versions demonstrating hot-reload.

### Implementation Roadmap

1. ✅ **Prove the concept** - Compiler integration working
2. ✅ **WASM hot-reload demo** - Technically feasible, demonstrated
3. ✅ **Phase 4: Visual UI** - Interactive component in browser with hot-reload
4. ✅ **Phase 5: AI integration** - Complete loop with automatic error retry
5. **Phase 6: Advanced safety** - Next focus areas:
   - State preservation across hot-reloads
   - Permission system with user dialogs
   - Component rollback/undo
   - Multi-component composition
   - Sandboxed execution
6. **Production readiness:**
   - WebSocket for no-refresh hot-reload
   - Better prompt engineering for reliable code generation
   - Streaming responses (see AI thinking)
   - Component library (common patterns)
7. **Real-world testing** - Deploy and iterate with actual users

## Contributing

This is early-stage research. If you're interested in:
- Self-modifying applications
- AI-assisted development
- Safe code generation
- Runtime compilation
- WASM sandboxing

...we'd love to hear your thoughts and ideas!

## Related Work

**Research that informed this:**
- Prototype V1 exploration of Rust-native UI patterns (see `archive/`)
- Analysis of Yew, Leptos, Dioxus frameworks
- Study of what makes frameworks worth existing
- Understanding of Rust's safety guarantees for UI development

**Existing frameworks we respect and learn from:**
- [Yew](https://yew.rs/) - Mature Rust web framework
- [Leptos](https://leptos.dev/) - Full-stack with fine-grained reactivity
- [Dioxus](https://dioxuslabs.com/) - Cross-platform Rust apps

## License

MIT OR Apache-2.0

## Name Origin

**Morpheus** - From Greek mythology, the god of dreams who could take any form. Fitting for applications that transform themselves through user imagination and AI collaboration.
