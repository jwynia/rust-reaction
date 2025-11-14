//! Visual Demo - Interactive Counter Component
//!
//! This is Version 2 of the counter component - GREEN THEME
//! Copy this file to src/lib.rs and rebuild to see hot-reload in action!

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

    /// Render the counter to the DOM - VERSION 2: GREEN THEME
    pub fn render(&self) {
        let display = get_element("counter-display");
        display.set_inner_html(&format!(
            r#"
            <div style="text-align: center; padding: 40px; font-family: system-ui;
                        background: linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%);">
                <h1 style="font-size: 72px; margin: 20px 0; color: #065f46;">
                    {}
                </h1>
                <p style="font-size: 18px; color: #047857; margin: 10px 0; font-weight: 600;">
                    💚 Success Counter (Eco Mode)
                </p>
                <div style="margin-top: 30px;">
                    <button onclick="counter.decrement()"
                            style="padding: 14px 28px; font-size: 18px; margin: 0 8px;
                                   background: #dc2626; color: white; border: none;
                                   border-radius: 8px; cursor: pointer; font-weight: 600;
                                   box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
                        ➖ Decrease
                    </button>
                    <button onclick="counter.reset()"
                            style="padding: 14px 28px; font-size: 18px; margin: 0 8px;
                                   background: #059669; color: white; border: none;
                                   border-radius: 8px; cursor: pointer; font-weight: 600;
                                   box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
                        🔄 Reset
                    </button>
                    <button onclick="counter.increment()"
                            style="padding: 14px 28px; font-size: 18px; margin: 0 8px;
                                   background: #047857; color: white; border: none;
                                   border-radius: 8px; cursor: pointer; font-weight: 600;
                                   box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
                        ➕ Increase
                    </button>
                </div>
                <div style="margin-top: 40px; padding: 24px; background: white;
                            border-radius: 12px; border: 3px solid #10b981;
                            box-shadow: 0 10px 30px rgba(0,0,0,0.1);">
                    <p style="margin: 0; color: #065f46; font-weight: 700; font-size: 18px;">
                        🧬 Morpheus Counter - Version 2 (Green Theme)
                    </p>
                    <p style="margin: 12px 0 0 0; color: #047857; font-size: 15px;">
                        This is what hot-reload looks like - same logic, new style!
                    </p>
                    <p style="margin: 8px 0 0 0; color: #6b7280; font-size: 13px; font-style: italic;">
                        AI generated this version with a green eco-friendly theme ♻️
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
    web_sys::console::log_1(&"🧬 Morpheus Counter Component V2 (Green) loaded!".into());
}
