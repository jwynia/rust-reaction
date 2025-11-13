# Code Examples: Existing Framework Patterns

## Classification
- **Domain:** Research
- **Stability:** Static
- **Abstraction:** Detailed
- **Confidence:** Established

## Purpose

Document specific code examples from existing frameworks to illustrate patterns and identify opportunities for more Rust-native approaches.

## Yew Examples

### Basic Component with State

```rust
use yew::{Callback, function_component, html, use_state};

#[function_component(UseState)]
fn state() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();  // ⚠️ Manual clone needed
        Callback::from(move |_| counter.set(*counter + 1))
    };
    html! {  // ⚠️ JSX-like macro
        <div>
            <button {onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}
```

**React Patterns Identified:**
- `use_state` hook (direct port from React)
- Order-dependent hooks
- Manual cloning for closures
- JSX-like syntax with html! macro
- Implicit component re-rendering

**Potential Rust-Native Alternatives:**
- Component as struct with explicit state
- Builder pattern instead of html! macro
- Ownership-based event binding
- Type-safe state updates

### Custom Hook with Effects

```rust
use yew::prelude::*;
use gloo::events::EventListener;
use gloo::utils::window;
use std::mem::drop;

#[function_component(ShowStorageChanged)]
pub fn show_storage_changed() -> Html {
    let state_storage_changed = use_state(|| false);
    {
        let state_storage_changed = state_storage_changed.clone();  // ⚠️ Clone again
        use_effect(|| {  // ⚠️ Effect closure pattern from React
            let listener = EventListener::new(
                &window(),
                "storage",
                move |_| state_storage_changed.set(true)
            );
            move || {  // ⚠️ Cleanup function (not RAII)
                drop(listener);
            }
        });
    }
    html! { /* ... */ }
}
```

**React Patterns:**
- use_effect with cleanup function
- Manual cleanup instead of RAII
- Clone-heavy event handling

**Rust-Native Alternatives:**
- Struct with Drop implementation for cleanup
- Lifetime-based effect management
- Event subscription as owned resource

## Leptos Examples

### Basic Component with Signals

```rust
use leptos::*;

#[component]
pub fn Input() -> impl IntoView {
    let (name, set_name) = create_signal(Some("Controlled".to_string()));
    // ⚠️ Tuple destructuring like SolidJS
    view! { /* ... */ }  // ⚠️ JSX-like macro
}
```

**SolidJS Patterns:**
- create_signal returns (getter, setter) tuple
- Similar to SolidJS API

**Rust-Native Alternatives:**
- Signal as struct with methods
- Generic over state type
- Builder-based reactivity

### Derived Signals

```rust
let (count, set_count) = create_signal(cx, 2);
let double_count = Signal::derive(cx, move || count() * 2);
// ⚠️ count() uses function call syntax for reactive read
```

**Patterns:**
- Function-call syntax for reads (JavaScript proxy-like)
- Explicit context parameter (cx)

**Rust-Native Alternatives:**
- Explicit .get() method
- Lens/optics pattern for derived state
- Type-based computation graph

## Dioxus Examples

### Component with use_signal

```rust
use dioxus::prelude::*;

#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    // ⚠️ `mut` for write access

    rsx! {  // ⚠️ JSX-like macro
        div {
            h1 { "Count: {count}" }
            button {
                onclick: move |_| count += 1,  // ⚠️ Nice ergonomics!
                "Increment"
            }
        }
    }
}
```

**Interesting Patterns:**
- `mut` binding for signals (more Rust-like!)
- `count += 1` instead of `count.set(count.get() + 1)`
- Still uses macro for view

**Rust-Native Opportunities:**
- This is closer! But could go further
- Builder pattern for elements
- Trait-based event handling

## Sycamore Examples

### Fine-Grained Reactivity

```rust
use sycamore::prelude::*;

#[component]
fn Counter<G: Html>() -> View<G> {
    let counter = create_signal(0);
    // ⚠️ Fine-grained reactivity like SolidJS

    view! {  // ⚠️ JSX macro again
        div {
            p { (counter.get()) }
            button(on:click=|_| counter.set(*counter.get() + 1)) {
                "Increment"
            }
        }
    }
}
```

**Patterns:**
- Explicit .get() and .set() (good!)
- Generic over render backend (good!)
- Still JSX-like

## Common Anti-Patterns (from Rust perspective)

### 1. Clone-Heavy Code

All frameworks require extensive cloning:
```rust
let counter = counter.clone();
let set_count = set_count.clone();
Callback::from(move |_| {
    // use cloned values
})
```

**Why this feels un-Rusty:**
- Fighting the borrow checker
- Not using ownership system
- Unclear ownership semantics

### 2. Order-Dependent Hooks

```rust
fn component() -> Html {
    let state1 = use_state(|| 0);  // Must be called in same order
    let state2 = use_state(|| "");  // every render
    if some_condition {
        let state3 = use_state(|| false);  // ⚠️ This breaks!
    }
}
```

**Why this feels un-Rusty:**
- Magic order dependency
- Not tracked by type system
- Runtime panics instead of compile errors

### 3. Macro-Heavy View Construction

All use JSX-like macros instead of Rust patterns:
```rust
html! { <div class="container"><p>{"text"}</p></div> }
view! { <div class="container"><p>{text}</p></div> }
rsx! { div { class: "container", p { "{text}" } } }
```

**Why this feels un-Rusty:**
- Not using builder pattern
- Not using method chaining
- Special syntax instead of regular Rust code

### 4. Implicit Runtime Behavior

Fine-grained reactivity tracks dependencies implicitly:
```rust
let derived = || count.get() * 2;  // Magically tracks count
```

**Why this feels un-Rusty:**
- Implicit behavior
- Not obvious from types
- Could use explicit dependency declaration

## Metadata
- **Created:** 2025-11-13
- **Last Updated:** 2025-11-13
- **Updated By:** Claude (AI Assistant)

## Relationships
- **Parent Nodes:** [research/overview.md]
- **Related Nodes:**
  - [design/rust-native-patterns.md] - contrasts - Shows alternative approaches
