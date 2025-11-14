# Component Library Evaluation for AI-Generated UIs

## Summary

Tested multiple Leptos component libraries for use with AI-generated self-modifying apps.

**Result:** Base Leptos 0.6 works excellently, but current component libraries have dependency conflicts.

## Test Results

### ✅ Base Leptos 0.6 (examples/leptos-poc)

**Status:** SUCCESS
**Compilation:** 36s dev mode
**Bundle:** 1.2MB unoptimized
**API:** Clean, AI-friendly

**Verdict:** **Use this as baseline**

### ❌ Leptonic 0.5

**Status:** FAILED - Dependency conflict
**Error:** `time` crate type annotation issue

```
error[E0282]: type annotations needed for `Box<_>`
  --> time-0.3.31/src/format_description/parse/mod.rs:83:9
```

**Verdict:** Wait for Leptonic to update dependencies

### ❌ Thaw 0.3

**Status:** FAILED - UUID dependency issue
**Error:** `uuid` crate feature flag conflict

```
error[E0433]: failed to resolve: could not find `RngImp` in `imp`
   --> uuid-1.18.1/src/rng.rs:18:10
```

**Verdict:** Dependency conflict with WASM target

### ❌ material-leptos

**Status:** NOT FOUND
**Error:** Package doesn't exist on crates.io

**Note:** The search result mentioned "leptos-material" but actual package name might be different or it's not published yet.

## Recommendations

### Option 1: Use Base Leptos + Tailwind CSS ⭐ RECOMMENDED

**Approach:**
- Use Leptos 0.6 for components and reactivity
- Use Tailwind CSS for styling
- AI generates components with Tailwind utility classes

**Example:**
```rust
view! {
    <div class="flex gap-4 justify-center">
        <button
            class="px-6 py-3 bg-red-500 text-white rounded-lg hover:bg-red-600"
            on:click=decrement>
            "- Decrement"
        </button>
        <button
            class="px-6 py-3 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
            on:click=increment>
            "+ Increment"
        </button>
    </div>
}
```

**Advantages:**
✅ No dependency conflicts
✅ Tailwind is well-known to AI (lots of training data)
✅ Consistent design system
✅ Small bundle size
✅ Fast compilation

**Disadvantages:**
⚠️ Longer class strings
⚠️ Need to include Tailwind CSS file

### Option 2: Build Custom Component Library

**Approach:**
- Create minimal set of core components
- Use Leptos 0.6 as base
- Style with CSS or Tailwind
- Keep it simple and dependency-light

**Components needed:**
- Button (with variants)
- Input/TextArea
- Card/Container
- Alert/Toast
- Modal
- Table
- Form elements

**Advantages:**
✅ Full control
✅ No dependency conflicts
✅ Optimized for AI generation
✅ Exactly what we need, nothing more

**Disadvantages:**
⚠️ Need to build and maintain
⚠️ Less battle-tested

### Option 3: Wait for Component Libraries to Mature

**Approach:**
- Start with Option 1 (Tailwind)
- Monitor component library updates
- Migrate when dependencies are resolved

**Timeline:**
- Leptonic might update for Leptos 0.6 compatibility
- Thaw might resolve uuid issues
- New libraries may emerge

## Proposed Path Forward

### Phase 1: Tailwind + Base Leptos (Now)

1. ✅ Use Leptos 0.6 (proven to work)
2. ⬜ Include Tailwind CSS in template
3. ⬜ Update AI prompt with Tailwind patterns
4. ⬜ Test AI generation quality

**Example AI Prompt Addition:**
```
Use Tailwind CSS utility classes for styling.
Common patterns:
- Buttons: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
- Containers: "max-w-2xl mx-auto p-6"
- Flexbox: "flex gap-4 items-center justify-center"
- Grid: "grid grid-cols-3 gap-4"
```

### Phase 2: Custom Components (Next 2-4 weeks)

Build minimal component library:

```rust
// examples/morpheus-components/src/button.rs
#[component]
pub fn Button(
    children: Children,
    on_click: impl Fn() + 'static,
    variant: ButtonVariant,
) -> impl IntoView {
    let classes = match variant {
        ButtonVariant::Primary => "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
        ButtonVariant::Danger => "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
        ButtonVariant::Secondary => "px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600",
    };

    view! {
        <button class=classes on:click=move |_| on_click()>
            {children()}
        </button>
    }
}
```

### Phase 3: Component Library Integration (Future)

When dependencies are resolved:
- Reevaluate Leptonic, Thaw
- Consider new libraries
- Migrate if beneficial

## Comparison Table

| Library | Status | Compilation | Bundle Size | AI-Friendly | Verdict |
|---------|--------|-------------|-------------|-------------|---------|
| **Leptos 0.6** | ✅ Works | 36s | 1.2MB | ✅ Excellent | **Use** |
| **+ Tailwind** | ✅ Ready | ~36s | +50KB | ✅ Very Good | **Recommended** |
| **Leptonic** | ❌ Broken | N/A | N/A | ? Unknown | Wait |
| **Thaw** | ❌ Broken | N/A | N/A | ? Unknown | Wait |
| **material-leptos** | ❌ Missing | N/A | N/A | ? Unknown | Not available |
| **Custom** | 🔄 To Build | TBD | TBD | ✅ Optimized | **Plan to build** |

## Files Created

### Working
- `examples/leptos-poc/` - ✅ Compiles successfully
  - Counter component with Leptos 0.6
  - Inline styles (temporary)
  - Full documentation

### Attempted
- `examples/thaw-poc/` - ❌ UUID dependency conflict
  - Fluent Design components
  - Dependencies need resolution

- `examples/material-poc/` - ❌ Package not found
  - Material Design attempt
  - Needs correct package name research

## Next Steps

### Immediate (This Week)
1. ⬜ Add Tailwind CSS to leptos-poc
2. ⬜ Test Tailwind + Leptos compilation
3. ⬜ Update AI prompt for Tailwind
4. ⬜ Test AI generation with Tailwind classes

### Short Term (Next 2 Weeks)
1. ⬜ Create morpheus-components crate
2. ⬜ Build 5-10 core components
3. ⬜ Document component API for AI
4. ⬜ Integrate into morpheus-complete

### Long Term (1-3 Months)
1. ⬜ Monitor component library updates
2. ⬜ Reevaluate Leptonic/Thaw
3. ⬜ Consider migration if better option emerges
4. ⬜ Expand custom component library as needed

## Conclusion

**Best path forward:** **Leptos 0.6 + Tailwind CSS**

**Reasoning:**
- ✅ Leptos proven to work excellently
- ✅ Tailwind is AI-friendly (lots of training data)
- ✅ No dependency conflicts
- ✅ Fast to implement
- ✅ Can build custom components on top
- ✅ Easy migration path when libraries mature

**Action:** Proceed with Tailwind integration into leptos-poc

---

**Date:** 2025-11-14
**Tested:** Leptos 0.6, Leptonic 0.5, Thaw 0.3
**Recommendation:** Tailwind + Base Leptos
**Status:** Ready to implement
