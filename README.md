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

**Current Phase:** Architecture & Proof of Concept

This project is actively exploring:
- Runtime Rust compilation for AI-generated code
- WASM component hot-reloading
- Safe sandboxing of untrusted components
- Transactional state management
- AI agent integration patterns

**Not production-ready yet.** This is research and development to prove the concept.

## Repository Structure

```
morpheus/
├── crates/
│   ├── morpheus-core/         # Core framework (coming soon)
│   ├── morpheus-compiler/     # Rust-to-WASM compiler integration
│   └── morpheus-runtime/      # Dynamic component loader & hot-reload
├── examples/
│   └── ai-modifiable-counter/ # Demo: counter that AI can enhance
├── context-network/           # Research, decisions, analysis
│   ├── research/              # Analysis of existing frameworks
│   ├── decisions/             # Key architectural decisions
│   ├── use-cases/             # Self-modifying apps analysis
│   └── planning/              # Implementation strategy
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

## Next Steps

1. **Prove the concept** - Build minimal compiler integration
2. **WASM hot-reload demo** - Show it's technically feasible
3. **AI integration** - Connect to LLM for code generation
4. **Basic UI** - Counter that AI can enhance
5. **Iterate** - Learn from real use

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
