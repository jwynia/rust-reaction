# Rust Reaction

A truly Rust-native frontend framework for building web UIs with WebAssembly.

## Philosophy

Rust Reaction explores what a frontend framework would look like if it **embraced Rust idioms** rather than translating JavaScript/TypeScript patterns into Rust syntax. Instead of making Rust "speak with a TypeScript accent," we leverage Rust's unique strengths:

- ✅ **Components as structs** - Not function components with hooks
- ✅ **Builder pattern** - Not JSX-like macros
- ✅ **Trait-based composition** - Not higher-order components
- ✅ **RAII for effects** - Not cleanup functions
- ✅ **Result for errors** - Not error boundaries
- ✅ **Type-state pattern** - For compile-time validation
- ✅ **Ownership-based lifecycle** - Not manual memory management
- ✅ **Explicit over implicit** - Clear ownership and reactivity

## Quick Example

```rust
use rust_reaction::prelude::*;

struct Counter {
    count: i32,
}

enum CounterMsg {
    Increment,
    Decrement,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Component for Counter {
    type Message = CounterMsg;

    fn view(&self) -> impl View {
        div()
            .class("counter")
            .child(
                button()
                    .text("Increment")
                    // Event handling without cloning!
            )
            .child(
                text(format!("Count: {}", self.count))
            )
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            CounterMsg::Increment => self.count += 1,
            CounterMsg::Decrement => self.count -= 1,
        }
    }
}
```

## How is this different from existing frameworks?

### vs Yew (React-inspired)

**Yew:**
```rust
#[function_component(Counter)]
fn counter() -> Html {
    let count = use_state(|| 0);  // ⚠️ Hooks (from React)
    let onclick = {
        let count = count.clone();  // ⚠️ Manual cloning
        Callback::from(move |_| count.set(*count + 1))
    };
    html! {  // ⚠️ JSX-like macro
        <button {onclick}>{ *count }</button>
    }
}
```

**Rust Reaction:**
```rust
struct Counter { count: i32 }

impl Component for Counter {
    type Message = Msg;

    fn view(&self) -> impl View {
        button()  // ✅ Builder pattern (regular Rust code)
            .text(format!("{}", self.count))
            // ✅ No cloning needed
    }

    fn update(&mut self, msg: Msg) {  // ✅ Direct mutation
        match msg {
            Msg::Increment => self.count += 1,
        }
    }
}
```

### vs Leptos (SolidJS-inspired)

**Leptos:**
```rust
#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);  // ⚠️ Tuple (from SolidJS)

    view! {  // ⚠️ JSX-like macro
        <button on:click=move |_| set_count.set(count() + 1)>
            {count}
        </button>
    }
}
```

**Rust Reaction:**
```rust
struct Counter {
    count: SharedState<i32>,  // ✅ Named state
}

impl Component for Counter {
    fn view(&self) -> impl View {
        button()  // ✅ Builder pattern
            .text(self.count.with(|c| format!("{}", c)))
            .on_click(|_| self.count.update(|c| *c += 1))
    }
}
```

### vs Dioxus (Cross-platform with signals)

**Dioxus:**
```rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);  // ⚠️ Hooks

    rsx! {  // ⚠️ JSX-like macro
        button { onclick: move |_| count += 1, "{count}" }
    }
}
```

**Rust Reaction:**
```rust
struct Counter { count: i32 }

impl Component for Counter {
    type Message = Msg;

    fn view(&self) -> impl View {
        button()  // ✅ Regular Rust, full IDE support
            .text(format!("{}", self.count))
    }
}
```

## Key Features

### 1. Builder Pattern for Views

No macros needed - just regular Rust code with full IDE support:

```rust
div()
    .class("container")
    .child(
        p().text("Hello, world!")
    )
    .child(
        button()
            .class("btn")
            .text("Click me")
            .on_click(|_| { /* handler */ })
    )
```

### 2. Type-Safe Attributes

Compile-time validation of HTML attributes:

```rust
// This compiles:
a().href("/home").class("link");

// This doesn't compile (div doesn't have href):
div().href("/home");  // ❌ Compile error!
```

### 3. RAII for Effects

Automatic cleanup using Rust's ownership:

```rust
struct ComponentWithEffect {
    _subscription: EventListener,  // Cleaned up automatically!
}
```

No manual cleanup functions needed.

### 4. Type-Safe Routing

Routes as enums, not strings:

```rust
enum Route {
    Home,
    User { id: u32 },
    Post { id: u32, slug: String },
}

// Type-safe navigation:
router.navigate(Route::User { id: 42 });

// Impossible to navigate to invalid route!
```

### 5. Result-Based Error Handling

Use Rust's error handling, not try-catch:

```rust
impl Component for DataDisplay {
    fn view(&self) -> Result<impl View, ViewError> {
        let data = self.fetch_data()?;  // ✅ Use ? operator

        Ok(div()
            .child(text(&data))
        )
    }
}
```

### 6. Trait-Based Composition

Use traits, not higher-order components:

```rust
trait Loadable {
    fn is_loading(&self) -> bool;
}

impl<T: Loadable + Component> Component for T {
    fn view(&self) -> impl View {
        if self.is_loading() {
            spinner()
        } else {
            self.render()
        }
    }
}
```

### 7. Iterators for Lists

Use Rust's iterator trait:

```rust
ul().children_from_iter(
    items.iter().map(|item|
        li().text(&item.name)
    )
)
```

## Project Status

⚠️ **This is a research prototype**, not a production-ready framework. The goal is to explore what truly Rust-native frontend patterns would look like and inspire the Rust web development community.

### What Works

- ✅ Core component model
- ✅ View builder pattern
- ✅ Basic state management
- ✅ RAII-based effects
- ✅ Type-safe routing (design)
- ✅ Example applications

### What's Missing

- ❌ Actual event handler integration
- ❌ Efficient DOM reconciliation
- ❌ Server-side rendering
- ❌ Advanced optimizations
- ❌ Full web_sys integration
- ❌ Production testing

## Examples

See the `examples/` directory for working demonstrations:

- **Counter** - Simple state management
- **Todo App** - Complex state, lists, and user input

To run an example:

```bash
cd examples/counter
wasm-pack build --target web
# Serve index.html with a local server
```

## Documentation

See the `context-network/` directory for:

- Research on existing frameworks
- Design philosophy and decisions
- Rust-native pattern proposals
- Comparison with existing frameworks

## Contributing

This is an exploratory project to demonstrate ideas. If you're interested in these patterns:

1. Explore the code and examples
2. Read the design documentation in `context-network/`
3. Consider how these ideas could inform existing frameworks
4. Share your thoughts and feedback

## License

MIT OR Apache-2.0

## Acknowledgments

Inspired by existing Rust frontend frameworks:
- [Yew](https://yew.rs/) - For showing Rust + WASM frontends are possible
- [Leptos](https://leptos.dev/) - For fine-grained reactivity
- [Dioxus](https://dioxuslabs.com/) - For ergonomic signals
- [Sycamore](https://sycamore.dev/) - For reactive primitives

And by frontend frameworks from other languages:
- React - For component-based architecture
- SolidJS - For fine-grained reactivity
- Svelte - For compile-time optimization

This project asks: "What if we started from Rust's strengths instead of translating JavaScript patterns?"
