//! Leptos Proof of Concept (Simplified - without Leptonic for now)
//!
//! Testing base Leptos compilation before adding component library.
//! Once this works, we'll add Leptonic.

use leptos::*;
use wasm_bindgen::prelude::*;

/// Counter component using basic Leptos (no component library yet)
#[component]
pub fn Counter() -> impl IntoView {
    // Leptos reactive state (no cx parameter in 0.6!)
    let (count, set_count) = create_signal(0);

    // Event handlers
    let increment = move |_| set_count.update(|n| *n += 1);
    let decrement = move |_| set_count.update(|n| *n -= 1);
    let reset = move |_| set_count.set(0);

    // View using basic Leptos HTML elements (no cx in 0.6!)
    view! {
        <div style="padding: 40px; max-width: 600px; margin: 0 auto;">
            <div style="margin-bottom: 30px;">
                <h1 style="font-size: 72px; text-align: center; color: #2563eb; margin: 20px 0;">
                    {move || count.get()}
                </h1>
                <p style="text-align: center; color: #64748b; font-size: 18px;">
                    "Counter Value"
                </p>
            </div>

            <div style="display: flex; gap: 10px; justify-content: center; margin-bottom: 30px;">
                <button
                    on:click=decrement
                    style="padding: 12px 24px; font-size: 16px; background: #ef4444; color: white;
                           border: none; border-radius: 6px; cursor: pointer;">
                    "- Decrement"
                </button>

                <button
                    on:click=reset
                    style="padding: 12px 24px; font-size: 16px; background: #6b7280; color: white;
                           border: none; border-radius: 6px; cursor: pointer;">
                    "Reset"
                </button>

                <button
                    on:click=increment
                    style="padding: 12px 24px; font-size: 16px; background: #10b981; color: white;
                           border: none; border-radius: 6px; cursor: pointer;">
                    "+ Increment"
                </button>
            </div>

            <div style="margin-top: 40px; padding: 20px; background: #f1f5f9;
                        border-radius: 8px; border-left: 4px solid #2563eb;">
                <p style="margin: 0; color: #1e293b; font-weight: 500;">
                    "🧬 Morpheus Counter - Leptos Edition"
                </p>
                <p style="margin: 8px 0 0 0; color: #64748b; font-size: 14px;">
                    "This component uses Leptos framework and can be hot-reloaded!"
                </p>
            </div>

            <div style="margin-top: 20px; padding: 20px; background: white;
                        border-radius: 8px; border: 1px solid #e2e8f0;">
                <h3 style="margin: 0 0 12px 0; color: #1e293b;">"Why This is Better"</h3>
                <ul style="margin: 0; padding-left: 20px; color: #64748b;">
                    <li>"✅ Structured Leptos components"</li>
                    <li>"✅ Reactive signals for state"</li>
                    <li>"✅ Type-safe event handlers"</li>
                    <li>"✅ Ready for component library (Leptonic, etc.)"</li>
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
