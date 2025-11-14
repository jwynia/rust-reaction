//! Material Design Component Library POC
//!
//! Testing material-leptos with Leptos 0.6

use leptos::*;
use material_leptos::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div style="padding: 40px;">
            <h1 style="font-size: 72px; text-align: center; color: #2563eb;">
                {move || count.get()}
            </h1>

            <p style="text-align: center; color: #64748b;">
                "Counter Value"
            </p>

            <div style="display: flex; gap: 10px; justify-content: center;">
                <MdButton
                    on_click=move |_| set_count.update(|n| *n -= 1)
                    label="- Decrement"/>

                <MdButton
                    on_click=move |_| set_count.set(0)
                    label="Reset"/>

                <MdButton
                    on_click=move |_| set_count.update(|n| *n += 1)
                    label="+ Increment"/>
            </div>

            <div style="margin-top: 40px;">
                <MdCard>
                    <h3>"Morpheus Counter - Material Design"</h3>
                    <p>"This uses Material Design components!"</p>
                </MdCard>
            </div>
        </div>
    }
}

#[wasm_bindgen]
pub fn mount() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <Counter/> })
}
