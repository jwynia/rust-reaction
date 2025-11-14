# Component Library Integration for AI-Generated UIs

## Classification
- **Domain:** Planning
- **Stability:** Evolving
- **Abstraction:** Detailed
- **Confidence:** Proposed

## Context

**Problem:** Currently, Morpheus AI generates bare HTML/CSS strings using `set_inner_html()`, meaning the AI reinvents UI primitives from scratch each time.

**Conclusion from v1:** Don't build another Rust frontend framework - use existing ones (Yew/Leptos/Dioxus).

**Current Need:** Integrate existing framework + component libraries so AI can compose pre-built, tested components instead of generating raw HTML.

## Current State (What We Have)

### AI System Prompt (morpheus-complete/src/main.rs:530-581)
```rust
fn create_system_prompt() -> String {
    r#"You are a Rust expert generating WebAssembly components using wasm-bindgen.

    Use wasm_bindgen for all browser interactions
    Use web_sys for DOM manipulation

    fn window() -> Window { web_sys::window().expect("no window") }
    fn document() -> Document { window().document().expect("no document") }

    pub fn render(&self) {
        let root = document().get_element_by_id("component-root")...;
        root.set_inner_html(&format!(/* raw HTML string */));
    }
    "#
}
```

### Current AI-Generated Component (visual-demo/src/lib.rs:66-110)
```rust
pub fn render(&self) {
    let display = get_element("counter-display");
    display.set_inner_html(&format!(
        r#"
        <div style="text-align: center; padding: 40px;">
            <h1 style="font-size: 72px;">{}</h1>
            <button onclick="counter.decrement()"
                    style="padding: 12px 24px; background: #ef4444;">
                - Decrement
            </button>
        </div>
        "#,
        self.count
    ));
}
```

**Problems:**
- AI generates inline styles every time
- No design system coherence
- Reinvents buttons, forms, layouts
- Error-prone (typos in HTML/CSS)
- Hard to maintain consistency

## Proposed State (What We Want)

### Framework Choice: **Leptos**

**Rationale:**
1. **Active ecosystem:** Largest component library selection (Thaw, Leptonic, leptos-material, shadcn/ui port)
2. **AI-friendly syntax:** `view!` macro is clear and structured
3. **Performance:** Fine-grained reactivity = smaller updates
4. **Component maturity:** Multiple production-ready UI libraries
5. **Compilation:** Good incremental compilation for faster AI iteration

**Alternatives Considered:**
- **Dioxus:** Good primitives, but fewer complete UI libraries; better for desktop apps
- **Yew:** Mature but less modern; smaller component ecosystem

### Proposed AI-Generated Component (with Leptos + Leptonic)
```rust
use leptos::*;
use leptonic::prelude::*;

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <Stack spacing=Size::Em(2.0) class="counter-container">
            <H1 style="font-size: 72px; text-align: center;">
                {count}
            </H1>

            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                <Button
                    on_press=move |_| set_count.update(|n| *n -= 1)
                    color=ButtonColor::Danger>
                    "- Decrement"
                </Button>

                <Button
                    on_press=move |_| set_count.set(0)
                    color=ButtonColor::Secondary>
                    "Reset"
                </Button>

                <Button
                    on_press=move |_| set_count.update(|n| *n += 1)
                    color=ButtonColor::Success>
                    "+ Increment"
                </Button>
            </Stack>

            <Alert variant=AlertVariant::Info>
                <AlertTitle>"Morpheus Counter - Version 1"</AlertTitle>
                <AlertDescription>
                    "This component uses Leptonic UI and can be hot-reloaded!"
                </AlertDescription>
            </Alert>
        </Stack>
    }
}
```

**Benefits:**
- ✅ Uses pre-built, tested components (`Button`, `Stack`, `Alert`)
- ✅ Consistent design system (Leptonic)
- ✅ Type-safe component props
- ✅ AI focuses on composition, not implementation
- ✅ Compile-time errors for incorrect usage

## Architecture Options

### Option A: Monolithic Compilation (Recommended for MVP)

**How it works:**
```
AI generates Rust code
    ↓
Code includes: use leptonic::prelude::*;
    ↓
SubprocessCompiler compiles everything together
    ↓
WASM bundle includes: Leptos + Leptonic + AI component
    ↓
Hot-reload into browser
```

**Cargo.toml for AI-generated components:**
```toml
[dependencies]
leptos = { version = "0.6", features = ["csr"] }
leptonic = "0.5"
wasm-bindgen = "0.2"
```

**Pros:**
- ✅ Simple architecture
- ✅ Works with current SubprocessCompiler
- ✅ No module linking complexity
- ✅ AI just imports what it needs

**Cons:**
- ⚠️ Slower compilation (full framework each time)
- ⚠️ Larger WASM bundles
- ⚠️ No shared code between versions

**Mitigation:**
- Leptos has good incremental compilation
- Can optimize later with Option B
- For now, prioritize working > fast

### Option B: Pre-compiled Component Library (Future Optimization)

**How it works:**
```
Component library compiled once
    ↓
Stored as base WASM module
    ↓
AI generates only component logic
    ↓
Link AI code with base module
    ↓
Hot-reload delta
```

**Pros:**
- ✅ Much faster iteration (only compile new code)
- ✅ Smaller individual component WASM
- ✅ Shared framework across versions

**Cons:**
- ⚠️ Complex WASM module linking
- ⚠️ Need to manage component library versioning
- ⚠️ More sophisticated build system

**Decision:** Start with Option A, migrate to Option B when compilation speed becomes bottleneck.

## Component Library Selection

### Phase 1: Leptonic (Recommended Starting Point)

**Why Leptonic:**
- Complete component set (buttons, forms, layouts, modals)
- Good documentation
- Active development
- Works well with Leptos CSR (client-side rendering)

**Available Components:**
- **Layout:** `Stack`, `Grid`, `Separator`
- **Inputs:** `Button`, `Input`, `TextArea`, `Select`, `Checkbox`, `Radio`, `Toggle`
- **Display:** `Alert`, `Card`, `Modal`, `Drawer`, `Tabs`, `Collapsible`
- **Feedback:** `ProgressBar`, `Spinner`, `Toast`, `Skeleton`
- **Data:** `Table`, `Pagination`
- **Navigation:** `AppBar`, `Drawer`, `Tabs`

### Phase 2: Additional Libraries (As Needed)

**leptos-material:** Material Design components
**Thaw:** Fluent Design components
**shadcn/ui port:** Tailwind-based components

**Strategy:** Start with one library (Leptonic), add others as mixins when needed.

## Implementation Changes

### 1. Update AI System Prompt

**Current:** Generate wasm-bindgen + web_sys code
**New:** Generate Leptos components with Leptonic UI

```rust
fn create_system_prompt() -> String {
    r#"You are a Rust expert generating Leptos components with Leptonic UI library.

CRITICAL RULES:
1. ONLY output Rust code - no explanations
2. Use Leptos for reactivity and components
3. Use Leptonic UI components for all UI elements
4. Always include: use leptos::*; use leptonic::prelude::*;
5. Components must be annotated with #[component]

TEMPLATE:

```rust
use leptos::*;
use leptonic::prelude::*;

#[component]
pub fn YourComponent(cx: Scope) -> impl IntoView {
    // State using signals
    let (state, set_state) = create_signal(cx, initial_value);

    // View using Leptonic components
    view! { cx,
        <Stack spacing=Size::Em(2.0)>
            <Button on_press=move |_| set_state.update(|n| *n + 1)>
                "Click me"
            </Button>
            <P>{state}</P>
        </Stack>
    }
}

// Export for WASM
#[wasm_bindgen]
pub fn mount() {
    mount_to_body(|cx| view! { cx, <YourComponent/> })
}
```

AVAILABLE LEPTONIC COMPONENTS:
- Layout: Stack, Grid, Separator
- Buttons: Button (with colors: Primary, Secondary, Success, Danger)
- Inputs: Input, TextArea, Select, Checkbox, Radio, Toggle
- Display: Alert, Card, Modal, Drawer, Tabs, H1, H2, P
- Feedback: ProgressBar, Spinner
- Data: Table

COMPONENT PATTERNS:
- Use <Stack> for vertical layouts
- Use <Stack orientation=StackOrientation::Horizontal> for horizontal
- Use <Button color=ButtonColor::Primary> for styled buttons
- Use create_signal for state
- Use on_press for button clicks
- Use {value} for displaying state

When you receive errors, ONLY output fixed code."#
        .to_string()
}
```

### 2. Update SubprocessCompiler Dependencies

**File:** `crates/morpheus-compiler/template/Cargo.toml`

```toml
[package]
name = "morpheus-component"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
leptos = { version = "0.6", features = ["csr", "nightly"] }
leptonic = { version = "0.5", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Window", "Document", "Element"] }

[profile.release]
opt-level = "z"  # Optimize for size
lto = true
codegen-units = 1
```

### 3. Update Component Template

**File:** `crates/morpheus-compiler/template/src/lib.rs`

```rust
// This file is replaced by AI-generated code, but this template shows the structure

use leptos::*;
use leptonic::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>"Replace this with AI-generated component"</div>
    }
}

#[wasm_bindgen]
pub fn mount() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
```

### 4. Update Frontend HTML to Call `mount()`

**File:** `examples/morpheus-complete/public/index.html`

```javascript
async function loadComponent(wasmBase64) {
    const wasmBytes = base64ToBytes(wasmBase64);
    const module = await WebAssembly.compile(wasmBytes);
    const instance = await WebAssembly.instantiate(module, {
        // wasm-bindgen imports
    });

    // Call Leptos mount function
    instance.exports.mount();
}
```

## Migration Path

### Phase 1: Proof of Concept (Week 1)
- ✅ Research complete (this document)
- [ ] Create `morpheus-leptos` example
- [ ] Update AI prompt for Leptos/Leptonic
- [ ] Test compilation with sample component
- [ ] Verify hot-reload works with Leptos

### Phase 2: Basic Integration (Week 2)
- [ ] Update SubprocessCompiler template dependencies
- [ ] Add Leptos/Leptonic to compilation pipeline
- [ ] Update frontend to mount Leptos apps
- [ ] Test with simple counter component

### Phase 3: AI Testing (Week 3)
- [ ] Test AI generation of various components
- [ ] Iterate on system prompt based on errors
- [ ] Build component examples library for AI reference
- [ ] Measure compilation speed

### Phase 4: Component Library Expansion (Week 4)
- [ ] Add more Leptonic components to AI's vocabulary
- [ ] Create component composition patterns
- [ ] Test complex UIs (forms, tables, modals)
- [ ] Document best practices

### Phase 5: Production Ready (Week 5+)
- [ ] Optimize compilation speed
- [ ] Add error recovery for bad AI code
- [ ] Create component preview system
- [ ] Add component library version management

## Expected Outcomes

### Developer Experience
**Before:**
```
User: "Create a todo list"
AI: Generates raw HTML with inline styles
Result: Works but inconsistent styling
```

**After:**
```
User: "Create a todo list"
AI: Generates Leptos component using Leptonic UI
Result: Professional UI with consistent design system
```

### Code Quality Comparison

**Before (390 lines with inline HTML):**
```rust
display.set_inner_html(&format!(
    r#"<div style="text-align: center; padding: 40px; font-family: system-ui;">
        <h1 style="font-size: 72px; margin: 20px 0; color: #2563eb;">{}</h1>
        <button onclick="counter.decrement()"
                style="padding: 12px 24px; font-size: 16px; margin: 0 8px;
                       background: #ef4444; color: white; border: none;
                       border-radius: 6px; cursor: pointer;">
            - Decrement
        </button>
    </div>"#,
    self.count
));
```

**After (~50 lines with components):**
```rust
view! { cx,
    <Stack spacing=Size::Em(2.0)>
        <H1>{count}</H1>
        <Button color=ButtonColor::Danger on_press=decrement>
            "- Decrement"
        </Button>
    </Stack>
}
```

### Maintainability
- ✅ AI-generated code is more readable
- ✅ Easier to debug (compile-time errors)
- ✅ Consistent design across components
- ✅ Leverages tested component library

### Performance
- ⚠️ Initial compilation: Slower (includes framework)
- ✅ Runtime: Faster (fine-grained reactivity)
- ✅ Bundle size: Optimizable with wasm-opt
- 🔄 Future: Pre-compiled base for fast iteration

## Open Questions

1. **Compilation Speed:** Will including Leptos make compilation too slow for AI iteration?
   - **Answer:** Test needed. Leptos has good incremental compilation.
   - **Mitigation:** Can use `--profile dev` for faster iteration, `--release` for final.

2. **WASM Bundle Size:** Will bundles be too large?
   - **Answer:** Modern WASM is well-optimized. Leptonic apps are reasonable size.
   - **Mitigation:** wasm-opt, compression, lazy loading.

3. **AI Learning Curve:** Can AI reliably generate Leptos code?
   - **Answer:** Test needed. Leptos syntax is structured and AI-friendly.
   - **Mitigation:** Comprehensive prompt with examples, error retry loop exists.

4. **Component Library Updates:** How to handle library version changes?
   - **Answer:** Pin versions initially, upgrade deliberately.
   - **Mitigation:** Version history system can handle component library version.

5. **Multiple Component Libraries:** Should we support mixing libraries?
   - **Answer:** Start with one (Leptonic), add carefully if needed.
   - **Mitigation:** Component libraries can be composed if compatible.

## Success Metrics

### Technical Metrics
- ✅ AI generates valid Leptos code >80% of first attempts
- ✅ Compilation time <10 seconds for typical components
- ✅ WASM bundle size <500KB compressed
- ✅ No inline HTML/CSS in AI-generated code
- ✅ UI consistency across AI-generated components

### User Experience Metrics
- ✅ Components look professional out-of-the-box
- ✅ Design system coherence across iterations
- ✅ Faster time from prompt to working UI
- ✅ Fewer "ugly" or "broken-looking" generations
- ✅ Users can request complex UIs (forms, tables) successfully

## Next Steps

1. **Get approval** on approach (Leptos + Leptonic)
2. **Create proof-of-concept** example
3. **Test AI generation** with updated prompt
4. **Measure compilation speed**
5. **Iterate based on results**

## Metadata
- **Created:** 2025-11-14
- **Last Updated:** 2025-11-14
- **Updated By:** Claude (AI Assistant)
- **Status:** Proposed - awaiting approval

## Relationships
- **Parent Nodes:** [planning/roadmap.md]
- **Related Nodes:**
  - [decisions/002-self-modifying-apps-pivot.md] - builds on
  - [decisions/001-rust-native-approach.md] - supersedes (in favor of existing frameworks)
  - [planning/implementation-summary.md] - next phase after
