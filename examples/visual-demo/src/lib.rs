//! Visual Demo - Interactive Counter Component
//!
//! This is Version 1 of the counter component.
//! It demonstrates a simple interactive UI that can be hot-reloaded.

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Window};

/// Get the window object
fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

/// Get the document object
fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

/// Get an element by ID
fn get_element(id: &str) -> Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("should have element with id '{}'", id))
}

/// Counter component state
#[wasm_bindgen]
pub struct Counter {
    count: i32,
}

#[wasm_bindgen]
impl Counter {
    /// Create a new counter
    #[wasm_bindgen(constructor)]
    pub fn new() -> Counter {
        Counter { count: 0 }
    }

    /// Increment the counter
    pub fn increment(&mut self) {
        self.count += 1;
        self.render();
    }

    /// Decrement the counter
    pub fn decrement(&mut self) {
        self.count -= 1;
        self.render();
    }

    /// Reset the counter
    pub fn reset(&mut self) {
        self.count = 0;
        self.render();
    }

    /// Get current count
    pub fn count(&self) -> i32 {
        self.count
    }

    /// Render the counter to the DOM
    pub fn render(&self) {
        let display = get_element("counter-display");
        display.set_inner_html(&format!(
            r#"
            <div style="text-align: center; padding: 40px; font-family: system-ui;">
                <h1 style="font-size: 72px; margin: 20px 0; color: #2563eb;">
                    {}
                </h1>
                <p style="font-size: 18px; color: #64748b; margin: 10px 0;">
                    Counter Value
                </p>
                <div style="margin-top: 30px;">
                    <button onclick="counter.decrement()"
                            style="padding: 12px 24px; font-size: 16px; margin: 0 8px;
                                   background: #ef4444; color: white; border: none;
                                   border-radius: 6px; cursor: pointer;">
                        - Decrement
                    </button>
                    <button onclick="counter.reset()"
                            style="padding: 12px 24px; font-size: 16px; margin: 0 8px;
                                   background: #6b7280; color: white; border: none;
                                   border-radius: 6px; cursor: pointer;">
                        Reset
                    </button>
                    <button onclick="counter.increment()"
                            style="padding: 12px 24px; font-size: 16px; margin: 0 8px;
                                   background: #10b981; color: white; border: none;
                                   border-radius: 6px; cursor: pointer;">
                        + Increment
                    </button>
                </div>
                <div style="margin-top: 40px; padding: 20px; background: #f1f5f9;
                            border-radius: 8px; border-left: 4px solid #2563eb;">
                    <p style="margin: 0; color: #1e293b; font-weight: 500;">
                        🧬 Morpheus Counter - Version 1
                    </p>
                    <p style="margin: 8px 0 0 0; color: #64748b; font-size: 14px;">
                        This component is running as WASM and can be hot-reloaded!
                    </p>
                </div>
            </div>
            "#,
            self.count
        ));
    }
}

/// Initialize the counter (called from JavaScript)
#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"🧬 Morpheus Counter Component loaded!".into());
}
