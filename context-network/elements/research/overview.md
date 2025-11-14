# Research: Existing Rust Frontend Frameworks

## Classification
- **Domain:** Research
- **Stability:** Dynamic
- **Abstraction:** Detailed
- **Confidence:** Established

## Purpose

This element contains research findings on existing Rust frontend frameworks to understand:
1. Current patterns and approaches
2. Where frameworks adopt JS/React patterns vs Rust idioms
3. Gaps in Rust-native approaches
4. Opportunities for a more idiomatic Rust framework

## Frameworks Analyzed

### Major Frameworks
1. **Yew** - Component-based, heavily React-inspired
2. **Leptos** - Fine-grained reactivity with signals
3. **Dioxus** - Cross-platform with hooks and signals
4. **Sycamore** - Fine-grained reactivity, similar to SolidJS

## Key Findings Summary

### Pattern Categories

#### 1. Component Model
- **Current approach**: Function components with macros (#[function_component], #[component])
- **React influence**: Strong - all frameworks use function components returning view descriptions
- **Rust idioms used**: Minimal - mostly macro-based syntax sugar

#### 2. State Management
- **Yew**: Hooks (use_state) - direct React port
- **Leptos**: Signals (create_signal) - inspired by SolidJS
- **Dioxus**: Signals (use_signal) - hybrid hooks + signals
- **Common pattern**: All return tuples or handles, require cloning for closures

#### 3. Reactivity
- **Yew**: Component-level (like React)
- **Leptos/Sycamore**: Fine-grained signals (like SolidJS)
- **Dioxus**: Signal-based with explicit .read()/.write()
- **Pattern**: Moving toward fine-grained reactivity, but with JS-inspired APIs

#### 4. View Layer
- **All frameworks**: Macro-based DSL (html!, view!, rsx!)
- **Pattern**: JSX-like syntax rather than Rust-native builders

## Analysis: JS Patterns vs Rust Idioms

### Where Frameworks Use JS Patterns

1. **Hooks Pattern** (Yew, Dioxus)
   - Borrowed from React
   - Order-dependent state loading
   - Not very Rust-like

2. **JSX-like Macros**
   - All frameworks use html!-style macros
   - Mimics JSX rather than using builder patterns

3. **Clone-Heavy Event Handlers**
   - Callbacks require manual cloning
   - Fighting the borrow checker
   - Pattern: Clone values into closures

4. **Implicit Reactivity Tracking** (Leptos, Sycamore)
   - Like SolidJS
   - Less explicit than typical Rust

### Where Frameworks Use Rust Idioms

1. **Type Safety**
   - Props are strongly typed structs
   - Compile-time checks for component props

2. **Error Handling** (limited)
   - Some use of Result types
   - Mostly still following JS pattern of panics

3. **Trait-Based Extension** (some frameworks)
   - Custom hooks can use traits
   - Component traits for lifecycle

## Gaps Identified

### Missing Rust-Native Patterns

1. **Builder Pattern for Views**
   - Instead of JSX macros, use method chaining
   - Example: `div().class("container").child(text("Hello"))`

2. **Ownership-Based Lifecycle**
   - Use Rust's ownership for component cleanup
   - RAII for effects instead of explicit cleanup functions

3. **Type-Driven Reactivity**
   - Use Rust's type system to prevent invalid state transitions
   - State machines instead of arbitrary state updates

4. **Trait-Based Composition**
   - Use traits for component capabilities
   - More flexible than inheritance or HOCs

5. **Result-Based Error Handling**
   - Proper error propagation instead of error boundaries
   - Leverage ? operator

6. **Iterator-Based Rendering**
   - Use Rust's iterator trait for lists
   - Lazy evaluation instead of eager rendering

## Metadata
- **Created:** 2025-11-13
- **Last Updated:** 2025-11-13
- **Updated By:** Claude (AI Assistant)

## Relationships
- **Parent Nodes:** [elements/README.md]
- **Related Nodes:**
  - [foundation/project_definition.md] - informs - Project goals
  - [decisions/] - influences - Design decisions
