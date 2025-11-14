# Reorganization Options for Pivot

## Current State

```
rust-reaction/
├── Cargo.toml                          # Workspace
├── PROJECT_README.md                   # Old vision docs
├── crates/rust-reaction/               # Prototype framework
│   ├── src/
│   │   ├── component.rs               # Component trait
│   │   ├── view.rs                    # Builder pattern
│   │   ├── state.rs                   # State management
│   │   ├── event.rs                   # RAII events
│   │   ├── routing.rs                 # Type-safe routing
│   │   └── dom.rs                     # DOM utils
├── examples/
│   ├── counter/                       # Static counter example
│   └── todo-app/                      # Static todo example
└── context-network/                   # All documentation
    ├── research/                      # Framework analysis
    ├── design/                        # Rust-native patterns
    ├── decisions/                     # Including new pivot!
    └── planning/                      # Evaluations, analysis
```

## What's Valuable vs What's Not

### Still Valuable ✅

From the prototype:
- **Component trait pattern** - Could adapt for dynamic components
- **View trait** - Builder pattern still useful
- **State management concepts** - Ownership model still applies
- **RAII effects** - EventListener pattern is solid
- **Type-safe routing** - Enum-based routing still good

From documentation:
- **All research** - Understanding existing frameworks
- **Pattern analysis** - Rust idioms still matter
- **Critical evaluation** - Lessons learned
- **New pivot docs** - Self-modifying apps vision

### Needs Rethinking ❌

- **ComponentHandle** - Too simple for hot-reload
- **Static examples** - Don't show self-modification
- **No compiler integration** - Critical missing piece
- **No WASM loading** - Need dynamic modules
- **No permissions system** - Need sandboxing

## Option 1: Archive and Start Fresh 📦

**Move old prototype to archive, clean slate for new architecture**

```
rust-reaction/
├── archive/
│   └── prototype-v1/          # Move everything here
│       ├── crates/
│       ├── examples/
│       └── README.md          # Old vision
├── crates/
│   ├── morpheus-core/         # New name, new architecture
│   ├── morpheus-compiler/     # Rust-to-WASM compiler
│   └── morpheus-runtime/      # Dynamic component loader
├── examples/
│   ├── self-modifying-counter/
│   └── ai-customizable-blog/
└── context-network/           # Keep all docs
    ├── research/
    ├── decisions/
    └── archive/
        └── v1-prototype/      # Link to archived code
```

**Pros:**
- ✅ Clean separation of old vs new
- ✅ Can reference old work without confusion
- ✅ Fresh start architecturally

**Cons:**
- ❌ Lose some good patterns (need to copy over)
- ❌ More work to set up
- ❌ Git history less obvious

## Option 2: Evolve In Place 🔄

**Keep existing structure, add new pieces alongside**

```
rust-reaction/
├── crates/
│   ├── rust-reaction-core/         # Rename: basic UI primitives
│   ├── rust-reaction-dynamic/      # NEW: dynamic loading
│   ├── rust-reaction-compiler/     # NEW: Rust compiler
│   └── rust-reaction-ai/           # NEW: AI integration
├── examples/
│   ├── static/                     # Move old examples here
│   │   ├── counter/
│   │   └── todo-app/
│   └── dynamic/                    # NEW: self-modifying examples
│       ├── ai-counter/
│       └── ai-customizable-app/
└── context-network/
    └── (keep all docs)
```

**Pros:**
- ✅ Preserve good work (Component, View, State)
- ✅ Less reorganization needed
- ✅ Can reuse existing patterns

**Cons:**
- ❌ Mixing two visions might be confusing
- ❌ Old examples don't showcase new capability
- ❌ Harder to explain what the project is

## Option 3: Hybrid - New Top-Level Project 🆕

**Create new project, reference old as dependency/inspiration**

```
morpheus/                       # NEW project
├── Cargo.toml
├── crates/
│   ├── morpheus-core/         # NEW: self-modifying app framework
│   ├── morpheus-compiler/     # NEW: compiler service
│   └── morpheus-runtime/      # NEW: dynamic runtime
├── examples/
│   └── ai-modifiable-apps/
└── context-network/
    └── (move relevant docs)

rust-reaction/                  # OLD project (archived or separate)
└── (keep as reference)
```

**Pros:**
- ✅ Cleanest separation
- ✅ Clear what each project is
- ✅ Can keep old project for reference

**Cons:**
- ❌ Most work to set up
- ❌ Lose connection between old research and new work
- ❌ More complex git history

## Option 4: Dual-Purpose Framework 🔀

**Framework supports both static AND dynamic use cases**

```
rust-reaction/
├── crates/
│   ├── rust-reaction/              # Core: Component, View, State
│   ├── rust-reaction-static/       # Static app builder
│   └── rust-reaction-dynamic/      # Self-modifying capability
├── examples/
│   ├── static-apps/
│   │   ├── counter/
│   │   └── todo/
│   └── dynamic-apps/
│       ├── ai-enhanced-counter/
│       └── self-modifying-blog/
└── context-network/
```

**Pros:**
- ✅ Reuse core abstractions
- ✅ Shows both use cases
- ✅ Less work

**Cons:**
- ❌ Confusing value proposition
- ❌ Two different target audiences
- ❌ Could end up doing neither well

## Recommendation

**Option 1: Archive and Start Fresh**

**Reasoning:**
1. **Clear focus** - Self-modifying apps are the unique value proposition
2. **Different architecture** - Need compiler, WASM loading, hot-reload
3. **Can cherry-pick** - Take good patterns from v1 (Component trait, RAII)
4. **Clean narrative** - Easy to explain what the project is
5. **Keep research** - All the analysis and learning is preserved

**Steps:**
1. Create `archive/prototype-v1/` folder
2. Move old code there: `crates/`, `examples/`, `PROJECT_README.md`
3. Update main README to reflect new vision
4. Start fresh with new architecture
5. Cherry-pick good patterns as needed (Component, View, etc.)

## What to Preserve from V1

### Patterns to Keep

1. **Component Trait Pattern**
```rust
pub trait Component {
    type Message;
    fn view(&self) -> impl View;
    fn update(&mut self, msg: Self::Message);
}
```
- Still valuable for dynamic components
- Adapt for hot-reload capability

2. **Builder Pattern for Views**
```rust
div()
    .class("container")
    .child(button().text("Click"))
```
- Less verbose than code generation
- Type-safe
- Could be used to preview AI-generated components

3. **RAII Event Listeners**
```rust
pub struct EventListener { /* ... */ }
impl Drop for EventListener {
    fn drop(&mut self) { /* cleanup */ }
}
```
- Perfect for component lifecycle
- Already Rust-native

4. **Type-Safe Routing**
```rust
enum Route {
    Home,
    User { id: u32 },
}
```
- Still applies to dynamic apps
- Could be AI-modifiable

5. **State Management Concepts**
```rust
pub struct State<T> {
    value: T,
    observers: Vec<Rc<dyn Fn(&T)>>,
}
```
- Ownership model still applies
- Need versioning for rollback

### What to Add New

1. **Component Loader (WASM)**
2. **Rust Compiler Integration**
3. **Hot-Reload System**
4. **AI Agent Integration**
5. **Permissions & Sandboxing**
6. **State Versioning & Rollback**
7. **Type Registry & Validation**

## Migration Script

```bash
#!/bin/bash

# Create archive directory
mkdir -p archive/prototype-v1

# Move old code
mv crates archive/prototype-v1/
mv examples archive/prototype-v1/
mv PROJECT_README.md archive/prototype-v1/README.md

# Create archive README
cat > archive/prototype-v1/ARCHIVE.md <<'EOF'
# Prototype V1 - Rust-Native UI Patterns

This was the initial exploration of Rust-native frontend patterns,
focusing on builder patterns, type safety, and RAII.

**Why archived:**
Project pivoted to self-modifying apps with LLM integration.

**What's valuable here:**
- Component trait pattern
- Builder pattern for views
- RAII event listeners
- Type-safe routing
- State management concepts

See context-network/decisions/002-self-modifying-apps-pivot.md
for the strategic pivot decision.
EOF

# Create new structure
mkdir -p crates/morpheus-core
mkdir -p crates/morpheus-compiler
mkdir -p crates/morpheus-runtime
mkdir -p examples/ai-modifiable-counter

echo "Archive complete! Old code in archive/prototype-v1/"
echo "Ready for new architecture."
```

## Alternative: Keep Both?

**Could also:**
- Keep `rust-reaction` branch as-is
- Create new `morpheus` branch for self-modifying apps
- Maintain both explorations

**Pro:** Can continue both paths
**Con:** Split effort, confusing which is "the project"

## Decision Point

**Before proceeding, decide:**
1. Archive old work? (Recommended)
2. New name? (morpheus, adaptive, flux, etc.)
3. Keep in same repo or new repo?
4. What patterns to cherry-pick from v1?

## Metadata
- **Created:** 2025-11-14
- **Last Updated:** 2025-11-14
- **Updated By:** Claude (AI Assistant)
