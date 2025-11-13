# Decision: Rust-Native Approach Over JS Pattern Translation

## Classification
- **Domain:** Core Decision
- **Stability:** Static
- **Abstraction:** Conceptual
- **Confidence:** Established

## Decision

Build a frontend framework that embraces Rust idioms rather than translating JavaScript/TypeScript patterns into Rust syntax.

## Context

Existing Rust frontend frameworks (Yew, Leptos, Dioxus, Sycamore) have achieved success by adapting familiar patterns from React, SolidJS, and other JavaScript frameworks. However, this approach makes developers "speak Rust with a TypeScript accent" rather than leveraging Rust's unique strengths.

## Options Considered

### Option 1: Follow Existing Patterns (React/SolidJS-style)
**Pros:**
- Familiar to web developers
- Proven patterns
- Lower learning curve for JS/TS developers

**Cons:**
- Fights Rust's ownership system (requires cloning)
- Doesn't leverage type system fully
- Macros instead of regular Rust code
- Order-dependent hooks (runtime errors)

### Option 2: Rust-Native Patterns (CHOSEN)
**Pros:**
- Leverages Rust's unique strengths
- Compile-time guarantees
- Natural to Rust developers
- No fighting the borrow checker
- Full IDE support (no macros for views)

**Cons:**
- Less familiar to web developers
- Learning curve for new patterns
- May require more verbose code in some cases

## Decision Rationale

We chose Option 2 because:

1. **Target Audience**: Rust developers building web UIs, not web developers learning Rust
2. **Long-term Value**: Teaching Rust-idiomatic patterns benefits the ecosystem
3. **Type Safety**: Rust's type system can catch UI bugs at compile time
4. **Ownership Model**: Can naturally express component lifecycles
5. **Community Contribution**: Exploring new approaches can inform existing frameworks

## Key Principles Established

1. **Components as Structs** - Not function components
2. **Builder Pattern** - Not JSX-like macros
3. **Traits for Composition** - Not HOCs
4. **RAII for Effects** - Not cleanup functions
5. **Result for Errors** - Not error boundaries
6. **Type-State Pattern** - For validation
7. **Ownership-Based Lifecycle** - Natural resource management
8. **Explicit over Implicit** - Clear dependencies

## Implementation Details

### Component Model
```rust
struct Counter { count: i32 }

impl Component for Counter {
    type Message = Msg;
    fn view(&self) -> impl View { /* ... */ }
    fn update(&mut self, msg: Msg) { /* ... */ }
}
```

### View Construction
```rust
div()
    .class("container")
    .child(button().text("Click"))
```

### State Management
```rust
State::new(value)
    .update(|v| *v += 1)
    .observe(|v| println!("{}", v))
```

### Effects
```rust
struct Component {
    _subscription: EventListener,  // RAII cleanup
}
```

### Routing
```rust
enum Route {
    Home,
    User { id: u32 },
}
router.navigate(Route::User { id: 42 });
```

## Trade-offs Accepted

1. **Verbosity**: Some patterns may be more verbose than macro-based approaches
2. **Familiarity**: Web developers need to learn new patterns
3. **Ecosystem**: May not integrate as easily with existing Rust web tools
4. **Performance**: Initial prototype prioritizes clarity over optimization

## Trade-offs Rejected

1. **Clone-heavy code**: We avoid manual cloning for callbacks
2. **Order-dependent hooks**: No magic ordering requirements
3. **Runtime errors**: Catch issues at compile time
4. **Macro magic**: Use regular Rust code where possible

## Impact

### On Components
- Structs with explicit state
- Methods for behavior
- Trait-based composition

### On State Management
- Explicit ownership
- Observable pattern
- Lens/optics for derived state

### On Views
- Builder pattern
- Type-safe attributes
- Iterator-based lists

### On Error Handling
- Result types
- ? operator support
- Pattern matching

## Success Metrics

1. ✅ Code feels natural to Rust developers
2. ✅ Type system catches UI bugs at compile time
3. ✅ No manual cloning required for callbacks
4. ✅ Full IDE support without macros
5. ✅ Clear ownership and lifecycle

## Related Decisions

- View builder pattern over macros
- RAII for effect cleanup
- Type-state for validation
- Enum-based routing

## Metadata
- **Created:** 2025-11-13
- **Last Updated:** 2025-11-13
- **Updated By:** Claude (AI Assistant)
- **Status:** Implemented in prototype

## Relationships
- **Parent Nodes:** [foundation/project_definition.md]
- **Related Nodes:**
  - [design/rust-native-patterns.md] - implements - Design patterns
  - [research/overview.md] - contrasts - Existing approaches
