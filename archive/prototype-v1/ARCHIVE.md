# Prototype V1 - Rust-Native UI Patterns (Archived)

This was the initial exploration of Rust-native frontend patterns, focusing on builder patterns, type safety, and RAII for traditional web applications.

## Why Archived

**Strategic Pivot**: The project pivoted to focus on **self-modifying applications with LLM integration** - a fundamentally different problem than building traditional web frameworks.

See `context-network/decisions/002-self-modifying-apps-pivot.md` for the full rationale.

## What Was Valuable Here

This exploration produced several patterns and insights that inform the new direction:

### Patterns Worth Preserving

1. **Component Trait Pattern** (`crates/rust-reaction/src/component.rs`)
   - Message-based updates
   - Clean separation of state and view
   - Can adapt for dynamic components

2. **Builder Pattern for Views** (`crates/rust-reaction/src/view.rs`)
   - Type-safe HTML construction
   - Method chaining
   - Compile-time attribute validation

3. **RAII Event Listeners** (`crates/rust-reaction/src/event.rs`)
   - Automatic cleanup via Drop
   - Ownership-based lifecycle
   - Perfect for component lifecycle management

4. **Type-Safe Routing** (`crates/rust-reaction/src/routing.rs`)
   - Enum-based routes
   - Pattern matching for navigation
   - Compile-time route validation

5. **State Management Concepts** (`crates/rust-reaction/src/state.rs`)
   - Ownership-based state
   - Observer pattern
   - Can extend for versioning/rollback

### Research That Remains Valuable

**Comprehensive Documentation** in `context-network/`:
- Analysis of existing frameworks (Yew, Leptos, Dioxus)
- Identification of JS patterns vs Rust idioms
- Critical evaluation of trade-offs
- Framework differentiation analysis

This research directly informed understanding of:
- What patterns work in Rust
- Where existing frameworks make compromises
- Why certain patterns are necessary
- What truly differentiates frameworks

## What This Prototype Demonstrated

### Successes ✅

- Type-safe HTML attributes work (compile-time validation)
- RAII for effects is elegant
- Builder pattern provides good IDE support
- Message-based updates feel natural in Rust
- Documentation quality was high

### Challenges ❌

- Builder pattern verbose compared to macros
- Event handlers still need component references (cloning issue)
- No actual reactivity connection to DOM
- Would need `Rc<RefCell<>>` for real implementation
- Competing with mature frameworks (Yew/Leptos/Dioxus) without differentiation

## The Key Insight

Building yet another web framework competing on implementation details (syntax, ergonomics) wasn't justified. **But building a framework for a fundamentally different use case - self-modifying apps with AI collaboration - absolutely is.**

## New Direction

The new framework focuses on:
- **Runtime compilation** of AI-generated components
- **Safe hot-reloading** without breaking the running app
- **WASM sandboxing** for untrusted code
- **Transactional rollback** of modifications
- **Type-checking as a safety gate** for AI-generated code

This solves the real problem: "How do I build apps that users can modify through AI conversation without breaking them?"

## How to Use This Archive

### Cherry-Picking Patterns

The new framework can reuse proven patterns:

```rust
// From prototype-v1/crates/rust-reaction/src/component.rs
pub trait Component {
    type Message;
    fn view(&self) -> impl View;
    fn update(&mut self, msg: Self::Message);
}

// Adapt for dynamic loading:
pub trait DynamicComponent {
    type Message;
    fn view(&self) -> View;  // Use trait object
    fn update(&mut self, msg: Self::Message);
    fn permissions(&self) -> Permissions;  // NEW: sandboxing
    fn to_wasm(&self) -> Vec<u8>;  // NEW: serialize for hot-reload
}
```

### Reference Implementation

Examples show working code for:
- Basic component structure
- State management
- Event handling
- View construction

These can inform new implementation without copying architectural mistakes.

## Files

```
archive/prototype-v1/
├── ARCHIVE.md                  # This file
├── README.md                   # Original vision (now superseded)
├── Cargo.toml                  # Workspace definition
├── crates/
│   └── rust-reaction/          # Core framework prototype
│       ├── src/
│       │   ├── lib.rs
│       │   ├── component.rs    # Component trait
│       │   ├── view.rs         # Builder pattern
│       │   ├── state.rs        # State management
│       │   ├── event.rs        # RAII events
│       │   ├── routing.rs      # Type-safe routing
│       │   └── dom.rs          # DOM utilities
│       └── Cargo.toml
└── examples/
    ├── counter/                # Simple counter app
    │   ├── src/lib.rs
    │   ├── index.html
    │   └── Cargo.toml
    └── todo-app/              # Todo list app
        ├── src/lib.rs
        ├── index.html
        └── Cargo.toml
```

## Metadata
- **Archived:** 2025-11-14
- **Original Development:** 2025-11-13 to 2025-11-14
- **Reason:** Strategic pivot to self-modifying apps
- **Value:** Patterns, research, and lessons learned
