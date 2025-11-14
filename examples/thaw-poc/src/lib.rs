//! Thaw Component Library POC
//!
//! Testing Thaw (Fluent Design) component library with Leptos 0.6

use leptos::*;
use thaw::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <Theme>
            <Space vertical=true>
                <h1 style="font-size: 72px; text-align: center; color: #2563eb;">
                    {move || count.get()}
                </h1>

                <p style="text-align: center; color: #64748b;">
                    "Counter Value"
                </p>

                <Space>
                    <Button
                        variant=ButtonVariant::Primary
                        on_click=move |_| set_count.update(|n| *n -= 1)>
                        "- Decrement"
                    </Button>

                    <Button
                        variant=ButtonVariant::Default
                        on_click=move |_| set_count.set(0)>
                        "Reset"
                    </Button>

                    <Button
                        variant=ButtonVariant::Primary
                        on_click=move |_| set_count.update(|n| *n += 1)>
                        "+ Increment"
                    </Button>
                </Space>

                <Alert title="Morpheus Counter - Thaw Edition">
                    "This component uses Thaw (Fluent Design) UI library!"
                </Alert>
            </Space>
        </Theme>
    }
}

#[wasm_bindgen]
pub fn mount() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <Counter/> })
}
