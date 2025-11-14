# Decision: Pivot to Self-Modifying Apps with LLM Integration

## Classification
- **Domain:** Core Decision
- **Stability:** Evolving
- **Abstraction:** Strategic
- **Confidence:** High

## Decision

**Pivot from "Rust-native patterns for web apps" to "Safe self-modifying applications with AI collaboration"**

## Context

Initial exploration focused on building a Rust-native frontend framework competing with Yew/Leptos/Dioxus. Critical evaluation revealed this didn't provide sufficient differentiation - we were offering implementation improvements, not solving a different problem.

**New Direction**: Building apps where LLM agents can safely modify the application code/UI in collaboration with users, while they're using the app.

## The Fundamental Problem This Solves

### User Need
"I want to build an app that users can modify through natural language conversation with an AI, without the app breaking during modification."

### Current Pain Point
TypeScript/JavaScript make it "far too easy to effectively break the app being used to self-modify":
- No runtime type safety
- No sandboxing
- Easy to corrupt state
- No rollback capability
- Can break the modification tool itself

### Why This is Different from Traditional Web Development

**Traditional Flow**:
```
Write code → Compile → Deploy → Run → (Done)
```

**Self-Modifying Flow**:
```
Run → User requests → AI generates → Verify → Hot-reload → Repeat
    ↑                                                          ↓
    └──────────────────────────────────────────────────────────┘
```

## Why This Justifies a New Framework

### Comparison with Existing Frameworks

**Yew/Leptos/Dioxus answer**: "How do I build web apps in Rust?"

**This framework answers**: "How do I build apps that safely modify themselves with AI?"

**Key Differentiators**:

| Aspect | Yew/Leptos/Dioxus | Self-Modifying Framework |
|--------|-------------------|-------------------------|
| **Use Case** | Static app development | Runtime AI modification |
| **Compilation** | Once, at build time | Continuously, at runtime |
| **Code Source** | Human developers | AI agents + humans |
| **Safety Model** | Compile-time only | Compile + runtime verification |
| **State** | Fixed schema | Migrating schema |
| **Components** | Bundled, fixed | Dynamic WASM modules |
| **Rollback** | Git/deploy | Real-time undo in app |
| **Sandboxing** | Not needed | Essential |
| **Type Checking** | Development time | Generation time |

### Platform Differentiation

**Yew**: Web SPAs (React-like)
**Leptos**: Full-stack web (Next.js-like)
**Dioxus**: Cross-platform (React Native-like)
**This**: **Self-modifying apps (completely new category)**

## What Rust Provides for This Use Case

### 1. **Compilation as Safety Gate**

AI-generated code must type-check before it can run:
```rust
// AI generates code
let ai_code = llm.generate("Add dark mode toggle");

// Framework compiles it
match compiler.compile(&ai_code) {
    Ok(wasm) => app.hot_reload(wasm),
    Err(errors) => {
        // Show user the errors
        // Ask AI to fix
        llm.fix_errors(&ai_code, &errors);
    }
}
```

**Value**: Type errors caught before breaking the app.

### 2. **Ownership for State Isolation**

Components can't accidentally corrupt each other:
```rust
pub struct Component {
    state: ComponentState,  // Private, owned
}

// AI-generated component CANNOT:
// - Access global state
// - Modify other components
// - Delete user data
```

**Value**: AI mistakes are contained.

### 3. **WASM for Sandboxing**

Components run in isolated sandbox:
```rust
let wasm = load_ai_component("new_feature.wasm");
let component = WasmComponent::new(wasm, Permissions {
    network: None,  // No network access
    storage: Limited(vec!["component-data"]),  // Limited storage
    apis: vec![],  // No special APIs
});
```

**Value**: Untrusted AI code can't steal data or attack the system.

### 4. **Type-Safe Hot Reload**

New versions must be compatible:
```rust
trait ComponentV1 {
    fn render(&self) -> View;
}

trait ComponentV2: ComponentV1 {  // Must still support V1!
    fn new_feature(&self);
}
```

**Value**: Updates won't break existing functionality.

### 5. **Transactional Updates**

All modifications are atomic and reversible:
```rust
// Take snapshot
let snapshot = app.snapshot();

// Apply AI modification
match app.apply(modification) {
    Ok(_) => app.commit(snapshot),
    Err(e) => {
        app.rollback(snapshot);  // Undo!
        show_error_to_user(e);
    }
}
```

**Value**: Bad modifications can be undone instantly.

## Architecture Vision

### Core Concept

```rust
pub struct SelfModifyingApp {
    // Dynamic components (loaded as WASM)
    components: ComponentRegistry,

    // Rust compiler (validates AI code)
    compiler: RustToWasmCompiler,

    // AI agent (generates modifications)
    ai: LLMAgent,

    // Versioned state (for rollback)
    state: VersionedState,

    // Type system (enforces compatibility)
    types: TypeRegistry,
}
```

### User Experience Flow

```
User: "Add a dark mode toggle"
  ↓
AI: Generates Rust code for toggle component
  ↓
Framework: Compiles to WASM (type-checks!)
  ↓
Framework: Shows preview in sandbox
  ↓
User: "Make it bigger"
  ↓
AI: Modifies the component
  ↓
Framework: Re-compiles, shows preview
  ↓
User: "Perfect! Apply it"
  ↓
Framework: Hot-reloads into live app
  ↓
User: "Actually, undo that"
  ↓
Framework: Rolls back atomically
```

### Safety Mechanisms

1. **Compile-Time Type Checking**
   - AI code must compile before running
   - Type errors shown to user (and AI)
   - No runtime type surprises

2. **Capability-Based Permissions**
   - Components declare what they need
   - Framework enforces limits
   - Sandboxed execution

3. **Version-Safe State**
   - Schema changes require migrations
   - Old data automatically migrated
   - No silent data corruption

4. **Atomic Transactions**
   - All changes are snapshottable
   - Rollback on any error
   - History of modifications

5. **Gradual Typing**
   - Start with dynamic (fast iteration)
   - Add types gradually (safety)
   - Compile to static (performance)

## Implementation Strategy

### Phase 1: Prove the Concept (Weeks 1-4)

**Goal**: Validate that Rust's safety helps

**Deliverables**:
1. TypeScript prototype showing failure modes
2. Rust prototype showing safety gates work
3. Benchmark compilation speed
4. Basic hot-reload demo

**Success Criteria**:
- Type-checking catches AI errors
- Hot-reload works without breaking app
- Compilation < 2 seconds for small components

### Phase 2: Core Framework (Months 1-3)

**Goal**: Build minimal viable framework

**Deliverables**:
1. Component loader (WASM)
2. Rust compiler integration
3. Hot-reload system
4. State snapshotting
5. Basic AI integration (OpenAI API)

**Success Criteria**:
- Can load AI-generated components
- Can undo modifications
- User can iterate with AI in real-time

### Phase 3: Safety & Polish (Months 3-6)

**Goal**: Production-ready safety

**Deliverables**:
1. Capability-based permissions
2. State migration system
3. Better error messages
4. Performance optimization
5. Documentation

**Success Criteria**:
- Components are properly sandboxed
- State changes don't corrupt data
- Fast enough for real use

### Phase 4: Developer Experience (Months 6+)

**Goal**: Delightful to use

**Deliverables**:
1. Visual component preview
2. Modification history UI
3. AI conversation interface
4. Templates and examples
5. Tutorial and guides

## Comparison: Before vs After Pivot

### Before: "Rust-Native UI Framework"

**Problem**: How to make Rust UI more idiomatic?
**Differentiation**: Implementation details (builder pattern, RAII, etc.)
**Competition**: Directly competing with Yew
**Value**: Marginal (slightly nicer syntax)
**Justification**: ❌ Insufficient

### After: "Self-Modifying Apps with AI"

**Problem**: How to let AI safely modify running apps?
**Differentiation**: Platform/use case (self-modification vs static)
**Competition**: No direct competitors in Rust
**Value**: Solves real problem TypeScript can't
**Justification**: ✅ Strong

## Risks and Mitigation

### Risk 1: Compilation Too Slow

**Problem**: Rust compilation might take too long for interactive use

**Mitigation**:
- Incremental compilation
- Precompiled templates
- Interpreted mode for iteration
- Aggressive caching

**Fallback**: Accept 5-10s for complex components

### Risk 2: AI Generates Bad Code

**Problem**: AI might generate code that compiles but is bad

**Mitigation**:
- Lint AI-generated code
- Performance budgets
- User review before apply
- Suggest improvements

**Fallback**: User can always undo

### Risk 3: State Migration Complexity

**Problem**: Changing state schemas might be too complex

**Mitigation**:
- Auto-generate simple migrations
- Provide migration helpers
- Fallback to default state
- Clear error messages

**Fallback**: Require manual migration for complex changes

### Risk 4: WASM Performance Overhead

**Problem**: WASM sandboxing might be too slow

**Mitigation**:
- Benchmark early
- Optimize hot paths
- Use native for trusted components
- Profile and improve

**Fallback**: Accept some overhead for safety

## Decision Rationale

### Why This is Worth Building

1. **Solves Real Problem**
   - TypeScript/JavaScript are unsafe for self-modifying apps
   - Rust's safety addresses this directly
   - Clear value proposition

2. **True Differentiation**
   - Not competing with Yew/Leptos/Dioxus
   - Different use case entirely
   - Complementary, not competitive

3. **Leverages Rust's Strengths**
   - Type system for AI code validation
   - Ownership for state isolation
   - WASM for sandboxing
   - This is what Rust is good at!

4. **Timely**
   - AI agents are getting powerful
   - Need for safe AI collaboration growing
   - Early mover advantage

5. **Can Build on Existing Work**
   - Use Leptos/Yew for base UI
   - Focus on dynamic loading layer
   - Reuse ecosystem

### Why Not Just Use TypeScript?

**TypeScript's Limitations**:
- No runtime type safety
- No true sandboxing
- Easy to corrupt state
- No compilation gate
- eval() is dangerous

**Rust's Advantages**:
- Compile AI code before running
- WASM sandboxing
- Ownership prevents corruption
- Type-checked at generation time
- Memory safety

## Success Metrics

### Technical Metrics

- ✅ AI-generated code must type-check before running
- ✅ Hot-reload without breaking app (>99% success rate)
- ✅ Rollback works atomically
- ✅ Components properly sandboxed
- ✅ Compilation < 5 seconds

### User Experience Metrics

- ✅ User can modify app through conversation
- ✅ Bad modifications don't break app
- ✅ User can undo any change
- ✅ Preview before apply
- ✅ Clear error messages

### Ecosystem Metrics

- ✅ Can integrate with existing Rust UI frameworks
- ✅ Reusable components
- ✅ Active community interest
- ✅ Real projects using it

## Open Questions

1. **How fast can we make Rust compilation?**
   - Need to benchmark
   - Might need incremental approach

2. **What's the right AI integration API?**
   - OpenAI? Anthropic? Local models?
   - How to handle context?

3. **How do we handle state migrations?**
   - Auto-generate? Manual? Hybrid?
   - What's the developer experience?

4. **Can we build on existing frameworks?**
   - Or do we need custom base?
   - How much can we reuse?

5. **What's the minimal viable product?**
   - Counter app that AI can modify?
   - Todo list with AI features?
   - Simple blog with AI customization?

## Next Steps

1. **Create TypeScript prototype** showing failure modes
2. **Build Rust proof-of-concept** showing safety works
3. **Benchmark compilation speed** for realistic components
4. **Design core architecture** in detail
5. **Build minimal demo** (AI-modifiable counter)

## Metadata
- **Created:** 2025-11-14
- **Last Updated:** 2025-11-14
- **Updated By:** Claude (AI Assistant)
- **Status:** Proposed - awaiting validation

## Relationships
- **Parent Nodes:** [decisions/001-rust-native-approach.md] - supersedes
- **Related Nodes:**
  - [use-cases/self-modifying-apps.md] - implements
  - [planning/framework-differentiation-analysis.md] - addresses
  - [planning/critical-evaluation.md] - responds to
