//! Counter Example
//!
//! This example demonstrates a simple counter component using Rust Reaction's
//! Rust-native patterns.

use rust_reaction::prelude::*;
use wasm_bindgen::prelude::*;

/// A simple counter component.
pub struct Counter {
    count: i32,
}

/// Messages that the counter can handle.
pub enum CounterMsg {
    Increment,
    Decrement,
    Reset,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn decrement(&mut self) {
        self.count -= 1;
    }

    fn reset(&mut self) {
        self.count = 0;
    }
}

impl Component for Counter {
    type Message = CounterMsg;

    fn view(&self) -> impl View {
        div()
            .class("counter-container")
            .child(
                div()
                    .class("counter-display")
                    .child(text(format!("Count: {}", self.count)))
            )
            .child(
                div()
                    .class("counter-controls")
                    .child(
                        button()
                            .class("btn btn-primary")
                            .text("Increment")
                    )
                    .child(
                        button()
                            .class("btn btn-secondary")
                            .text("Decrement")
                    )
                    .child(
                        button()
                            .class("btn btn-danger")
                            .text("Reset")
                    )
            )
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            CounterMsg::Increment => self.increment(),
            CounterMsg::Decrement => self.decrement(),
            CounterMsg::Reset => self.reset(),
        }
    }
}

/// Entry point for the WASM module.
#[wasm_bindgen(start)]
pub fn run() {
    // Set up panic hook for better error messages
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let counter = Counter::new();
    mount_to_body(counter);
}
