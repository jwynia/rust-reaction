# Rust-Native Frontend Patterns

## Classification
- **Domain:** Design
- **Stability:** Evolving
- **Abstraction:** Conceptual
- **Confidence:** Speculative

## Purpose

Define what truly Rust-native frontend patterns would look like, embracing Rust idioms rather than translating JavaScript patterns.

## Core Philosophy

**Rust's Strengths to Leverage:**
1. **Ownership & Borrowing** - Use for lifecycle management
2. **Type System** - Prevent invalid states at compile time
3. **Traits** - Flexible composition without inheritance
4. **Error Handling** - Result types instead of exceptions
5. **Iterators** - Lazy, composable sequences
6. **RAII** - Automatic resource cleanup
7. **Zero-Cost Abstractions** - Performance without runtime overhead
8. **Pattern Matching** - Explicit state handling

## 1. Component Model

### Current (React-Style):
```rust
#[function_component(Counter)]
fn counter() -> Html {
    let count = use_state(|| 0);
    html! { /* ... */ }
}
```

### Rust-Native Approach:

#### Option A: Components as Structs with Builders

```rust
struct Counter {
    count: Cell<i32>,
}

impl Counter {
    fn new() -> Self {
        Self { count: Cell::new(0) }
    }

    fn increment(&self) {
        self.count.set(self.count.get() + 1);
    }
}

impl Component for Counter {
    fn view(&self) -> impl View {
        div()
            .class("counter")
            .child(
                button()
                    .text("Increment")
                    .on_click(|| self.increment())
            )
            .child(
                text(format!("Count: {}", self.count.get()))
            )
    }
}
```

**Rust Idioms Used:**
- ✅ Struct-based state (explicit, named fields)
- ✅ Methods for behavior
- ✅ Builder pattern for view construction
- ✅ Trait for component interface
- ✅ Interior mutability (Cell) when appropriate

#### Option B: Type-State Pattern

```rust
struct Counter<S> {
    count: i32,
    _state: PhantomData<S>,
}

struct Idle;
struct Loading;

impl Counter<Idle> {
    fn new() -> Self {
        Counter { count: 0, _state: PhantomData }
    }

    fn increment(self) -> Counter<Idle> {
        Counter { count: self.count + 1, _state: PhantomData }
    }

    fn start_loading(self) -> Counter<Loading> {
        Counter { count: self.count, _state: PhantomData }
    }
}

impl Counter<Loading> {
    fn finish_loading(self) -> Counter<Idle> {
        Counter { count: self.count, _state: PhantomData }
    }
}
```

**Rust Idioms Used:**
- ✅ Type-state pattern - invalid states are unrepresentable
- ✅ Compile-time state machine
- ✅ Move semantics for state transitions

## 2. View Construction

### Current (JSX-Style):
```rust
html! {
    <div class="container">
        <p>{"Hello"}</p>
    </div>
}
```

### Rust-Native: Builder Pattern

```rust
div()
    .class("container")
    .child(
        p().text("Hello")
    )
```

**Benefits:**
- ✅ Regular Rust code (IDE support, refactoring, etc.)
- ✅ Method chaining (familiar Rust pattern)
- ✅ Type-safe attribute builders
- ✅ No macro magic

### Advanced: Type-Safe Attributes

```rust
trait HasClass {
    fn class(self, class: impl Into<String>) -> Self;
}

trait HasHref {
    fn href(self, href: impl Into<String>) -> Self;
}

impl<T> HasClass for Div<T> { /* ... */ }
impl<T> HasHref for A<T> { /* ... */ }

// This compiles:
div().class("container");
a().href("/home").class("link");

// This doesn't compile (div doesn't have href):
// div().href("/home");  // ❌ Compile error!
```

**Rust Idioms:**
- ✅ Trait-based capabilities
- ✅ Compile-time validation
- ✅ No invalid HTML structure possible

## 3. State Management

### Current (Hooks/Signals):
```rust
let (count, set_count) = create_signal(0);
set_count.set(count.get() + 1);
```

### Rust-Native: Owned State with Observers

```rust
struct State<T> {
    value: T,
    observers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> State<T> {
    fn new(value: T) -> Self {
        State { value, observers: Vec::new() }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn update(&mut self, f: impl FnOnce(&mut T)) {
        f(&mut self.value);
        for observer in &self.observers {
            observer(&self.value);
        }
    }

    fn map<U>(&self, f: impl Fn(&T) -> U) -> U {
        f(&self.value)
    }

    fn observe(&mut self, f: impl Fn(&T) + 'static) {
        self.observers.push(Box::new(f));
    }
}

// Usage:
let mut counter = State::new(0);
counter.observe(|count| {
    println!("Count changed: {}", count);
});
counter.update(|c| *c += 1);
```

**Rust Idioms:**
- ✅ Explicit ownership
- ✅ Borrowing rules enforced
- ✅ Observer pattern (classic Rust)
- ✅ Functional transformations with map

### Alternative: Lens Pattern for Derived State

```rust
trait Lens<T, U> {
    fn view(&self, source: &T) -> &U;
    fn update(&self, source: &mut T, value: U);
}

struct CountDoubled;

impl Lens<Counter, i32> for CountDoubled {
    fn view(&self, counter: &Counter) -> &i32 {
        // Return reference to computed value
        &(counter.count * 2)  // Simplified
    }

    fn update(&self, counter: &mut Counter, value: i32) {
        counter.count = value / 2;
    }
}
```

**Rust Idioms:**
- ✅ Optics/Lens pattern (functional programming in Rust)
- ✅ Type-safe projections
- ✅ Bidirectional transformations

## 4. Effects & Side Effects

### Current (Effect Hooks):
```rust
use_effect(|| {
    let subscription = subscribe();
    move || drop(subscription)  // Cleanup
});
```

### Rust-Native: RAII with Drop

```rust
struct Subscription {
    id: String,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        unsubscribe(&self.id);
    }
}

struct ComponentWithEffect {
    _subscription: Subscription,  // Automatically cleaned up!
}

impl ComponentWithEffect {
    fn new() -> Self {
        Self {
            _subscription: Subscription { id: "sub-1".into() }
        }
    }
}
```

**Rust Idioms:**
- ✅ RAII - automatic cleanup
- ✅ Ownership-based lifecycle
- ✅ No manual cleanup functions needed

### Alternative: Explicit Effect as Struct

```rust
trait Effect {
    type Output;
    fn run(&self) -> Self::Output;
    fn cleanup(&self, output: Self::Output);
}

struct WindowResizeEffect {
    callback: Box<dyn Fn(i32, i32)>,
}

impl Effect for WindowResizeEffect {
    type Output = EventListener;

    fn run(&self) -> EventListener {
        EventListener::new(/* ... */)
    }

    fn cleanup(&self, listener: EventListener) {
        drop(listener);
    }
}
```

**Rust Idioms:**
- ✅ Trait-based abstraction
- ✅ Explicit lifecycle
- ✅ Type-safe effect outputs

## 5. Event Handling

### Current (Callback with Cloning):
```rust
let count = count.clone();
Callback::from(move |_| count.set(*count + 1))
```

### Rust-Native: Method References

```rust
button()
    .on_click(Counter::increment)  // Method reference
```

Or with more complex logic:

```rust
struct TodoList {
    todos: Vec<String>,
}

impl TodoList {
    fn add_todo(&mut self, text: String) {
        self.todos.push(text);
    }

    fn view(&self) -> impl View {
        div().child(
            button()
                .text("Add")
                .on_click_with_arg(
                    Self::add_todo,
                    "New todo".into()
                )
        )
    }
}
```

**Rust Idioms:**
- ✅ No manual cloning
- ✅ Clear ownership
- ✅ Method references (like Rust closures)

## 6. Lists & Iteration

### Current (Macro-Based):
```rust
html! {
    <ul>
        { for items.iter().map(|item| html! { <li>{item}</li> }) }
    </ul>
}
```

### Rust-Native: Iterator Trait

```rust
ul().children_from_iter(
    items.iter().map(|item|
        li().text(item)
    )
)
```

Or with keyed elements:

```rust
trait KeyedView {
    type Key: Hash + Eq;
    fn key(&self) -> Self::Key;
}

ul().keyed_children(
    items.iter().map(|item|
        (item.id, li().text(&item.text))
    )
)
```

**Rust Idioms:**
- ✅ Iterator trait usage
- ✅ Lazy evaluation
- ✅ Type-safe keys
- ✅ No special syntax needed

## 7. Error Handling

### Current (Error Boundaries - from React):
```rust
html! {
    <ErrorBoundary>
        <MayFailComponent />
    </ErrorBoundary>
}
```

### Rust-Native: Result Types

```rust
impl Component for DataDisplay {
    fn view(&self) -> Result<impl View, ViewError> {
        let data = self.fetch_data()?;

        Ok(div()
            .child(text(&data))
        )
    }
}

// In parent:
div().child(
    match child.view() {
        Ok(view) => view,
        Err(e) => error_view(e),
    }
)
```

Or with more ergonomics:

```rust
div().child(
    data_display.view()
        .unwrap_or_else(|e| error_view(e))
)
```

**Rust Idioms:**
- ✅ Result types for errors
- ✅ ? operator for propagation
- ✅ Explicit error handling
- ✅ Pattern matching

## 8. Component Composition

### Current (HOCs/Props):
```rust
fn with_loading<P>(component: Component<P>) -> Component<P> {
    // Wrap component
}
```

### Rust-Native: Traits

```rust
trait Loadable {
    fn is_loading(&self) -> bool;
}

trait Renderable {
    fn render(&self) -> impl View;
}

impl<T: Loadable + Renderable> Component for T {
    fn view(&self) -> impl View {
        if self.is_loading() {
            spinner()
        } else {
            self.render()
        }
    }
}
```

**Rust Idioms:**
- ✅ Trait-based composition
- ✅ Generic implementations
- ✅ Zero-cost abstractions
- ✅ No wrapper components

## 9. Routing

### Current (String-Based):
```rust
Router::new()
    .route("/", home)
    .route("/about", about)
```

### Rust-Native: Type-Safe Routes

```rust
#[derive(Debug, Clone)]
enum Route {
    Home,
    About,
    User { id: u32 },
    Post { id: u32, slug: String },
}

impl Route {
    fn to_path(&self) -> String {
        match self {
            Route::Home => "/".into(),
            Route::About => "/about".into(),
            Route::User { id } => format!("/user/{}", id),
            Route::Post { id, slug } => format!("/post/{}/{}", id, slug),
        }
    }

    fn from_path(path: &str) -> Result<Self, RouteError> {
        // Parse path into Route enum
    }
}

impl Component for App {
    fn view(&self) -> impl View {
        match self.current_route {
            Route::Home => home_view(),
            Route::About => about_view(),
            Route::User { id } => user_view(id),
            Route::Post { id, slug } => post_view(id, slug),
        }
    }
}

// Navigation:
fn navigate_to_user(id: u32) {
    router.navigate(Route::User { id });
    // Impossible to navigate to invalid route!
}
```

**Rust Idioms:**
- ✅ Enum for route variants
- ✅ Pattern matching for routing
- ✅ Type-safe navigation
- ✅ Compile-time route validation

## 10. Forms & Validation

### Current (Controlled Components):
```rust
let (value, set_value) = create_signal(String::new());

input()
    .value(&value.get())
    .on_input(move |e| set_value.set(e.target.value))
```

### Rust-Native: Type-Driven Validation

```rust
use std::marker::PhantomData;

struct Validated;
struct Unvalidated;

struct FormField<T, S> {
    value: T,
    error: Option<String>,
    _state: PhantomData<S>,
}

impl<T> FormField<T, Unvalidated> {
    fn new(value: T) -> Self {
        FormField {
            value,
            error: None,
            _state: PhantomData
        }
    }

    fn validate(self, validator: impl Fn(&T) -> Result<(), String>)
        -> Result<FormField<T, Validated>, FormField<T, Unvalidated>>
    {
        match validator(&self.value) {
            Ok(()) => Ok(FormField {
                value: self.value,
                error: None,
                _state: PhantomData,
            }),
            Err(e) => Err(FormField {
                value: self.value,
                error: Some(e),
                _state: PhantomData,
            }),
        }
    }
}

impl<T> FormField<T, Validated> {
    fn value(&self) -> &T {
        &self.value
    }
}

// Usage:
struct LoginForm {
    email: FormField<String, Unvalidated>,
}

impl LoginForm {
    fn submit(self) -> Result<(), Self> {
        let email = self.email.validate(|e| {
            if e.contains('@') {
                Ok(())
            } else {
                Err("Invalid email".into())
            }
        }).map_err(|invalid_field| {
            LoginForm { email: invalid_field }
        })?;

        // email is now proven valid at compile time!
        send_login_request(email.value());
        Ok(())
    }
}
```

**Rust Idioms:**
- ✅ Type-state pattern for validation
- ✅ Compile-time validation tracking
- ✅ Invalid states unrepresentable
- ✅ Result for validation outcome

## Summary: Rust-Native Principles

1. **Use structs, not functions** - Components as data types
2. **Builder pattern, not macros** - For view construction
3. **Traits, not HOCs** - For composition
4. **RAII, not cleanup functions** - For resource management
5. **Result, not boundaries** - For error handling
6. **Enums, not strings** - For variants and routes
7. **Type-state, not runtime checks** - For state machines
8. **Ownership, not cloning** - For lifecycle
9. **Iterators, not special syntax** - For lists
10. **Explicit, not implicit** - For reactive dependencies

## Metadata
- **Created:** 2025-11-13
- **Last Updated:** 2025-11-13
- **Updated By:** Claude (AI Assistant)

## Relationships
- **Parent Nodes:** [elements/design/]
- **Related Nodes:**
  - [research/code-examples.md] - contrasts - Shows existing patterns
  - [research/overview.md] - addresses - Gaps in existing frameworks
