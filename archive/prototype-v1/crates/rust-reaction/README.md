# rust-reaction

Core library for the Rust Reaction framework.

See the main [project README](../../README.md) for documentation and examples.

## Module Overview

- **component** - Component trait and mounting
- **view** - Builder pattern for DOM construction
- **state** - Observable state management
- **event** - RAII-based event handling
- **routing** - Type-safe routing with enums
- **dom** - DOM utilities

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rust-reaction = { path = "path/to/rust-reaction" }
```

Import the prelude:

```rust
use rust_reaction::prelude::*;
```

## Design Principles

1. **Explicit over implicit** - No magic behavior
2. **Type safety** - Leverage Rust's type system
3. **Zero-cost abstractions** - No runtime overhead
4. **Ownership-based** - Use Rust's ownership for lifecycle
5. **No macros for views** - Builder pattern instead
