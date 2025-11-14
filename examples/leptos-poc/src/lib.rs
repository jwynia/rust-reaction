//! Leptos + Tailwind CSS Proof of Concept
//!
//! Demonstrates AI-friendly component generation using:
//! - Leptos 0.6 for reactive components
//! - Tailwind CSS for utility-first styling
//! - No inline styles - all styling via Tailwind classes

use leptos::*;
use wasm_bindgen::prelude::*;

/// Counter component using Leptos + Tailwind CSS
#[component]
pub fn Counter() -> impl IntoView {
    // Leptos reactive state
    let (count, set_count) = create_signal(0);

    // Event handlers
    let increment = move |_| set_count.update(|n| *n += 1);
    let decrement = move |_| set_count.update(|n| *n -= 1);
    let reset = move |_| set_count.set(0);

    // View using Tailwind utility classes
    view! {
        <div class="p-10 max-w-2xl mx-auto">
            // Counter Display
            <div class="mb-8">
                <h1 class="text-7xl text-center text-blue-600 my-5">
                    {move || count.get()}
                </h1>
                <p class="text-center text-slate-600 text-lg">
                    "Counter Value"
                </p>
            </div>

            // Button Controls
            <div class="flex gap-3 justify-center mb-8">
                <button
                    on:click=decrement
                    class="px-6 py-3 text-base bg-red-500 text-white border-0 rounded-lg cursor-pointer hover:bg-red-600 transition-colors">
                    "- Decrement"
                </button>

                <button
                    on:click=reset
                    class="px-6 py-3 text-base bg-gray-500 text-white border-0 rounded-lg cursor-pointer hover:bg-gray-600 transition-colors">
                    "Reset"
                </button>

                <button
                    on:click=increment
                    class="px-6 py-3 text-base bg-green-500 text-white border-0 rounded-lg cursor-pointer hover:bg-green-600 transition-colors">
                    "+ Increment"
                </button>
            </div>

            // Info Alert
            <div class="mt-10 p-5 bg-slate-100 rounded-lg border-l-4 border-blue-600">
                <p class="m-0 text-slate-900 font-medium">
                    "🧬 Morpheus Counter - Leptos + Tailwind Edition"
                </p>
                <p class="mt-2 mb-0 text-slate-600 text-sm">
                    "This component uses Leptos framework with Tailwind CSS and can be hot-reloaded!"
                </p>
            </div>

            // Benefits Card
            <div class="mt-5 p-5 bg-white rounded-lg border border-slate-200">
                <h3 class="mt-0 mb-3 text-slate-900">"Why This is Better"</h3>
                <ul class="m-0 pl-5 text-slate-600">
                    <li>"✅ Structured Leptos components"</li>
                    <li>"✅ Reactive signals for state"</li>
                    <li>"✅ Type-safe event handlers"</li>
                    <li>"✅ Tailwind CSS utility classes (AI-friendly!)"</li>
                    <li>"✅ No inline styles - clean and maintainable"</li>
                </ul>
            </div>
        </div>
    }
}

/// Mount the Leptos app
#[wasm_bindgen]
pub fn mount() {
    // Better panic messages in the browser console
    console_error_panic_hook::set_once();

    // Mount the Leptos app to the body (no cx in 0.6!)
    leptos::mount_to_body(|| view! { <Counter/> })
}
