//! Visual Demo - Interactive Counter Component
//!
//! This is Version 3 of the counter component - ANIMATED THEME
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

    /// Render the counter to the DOM - VERSION 3: ANIMATED
    pub fn render(&self) {
        let display = get_element("counter-display");

        // Determine color based on count value
        let (bg_color, text_color) = if self.count > 0 {
            ("#10b981", "#065f46")
        } else if self.count < 0 {
            ("#ef4444", "#991b1b")
        } else {
            ("#6366f1", "#312e81")
        };

        display.set_inner_html(&format!(
            r#"
            <style>
                @keyframes pulse {{
                    0%, 100% {{ transform: scale(1); }}
                    50% {{ transform: scale(1.05); }}
                }}
                @keyframes slideIn {{
                    from {{ opacity: 0; transform: translateY(-20px); }}
                    to {{ opacity: 1; transform: translateY(0); }}
                }}
                @keyframes rainbow {{
                    0% {{ border-color: #ef4444; }}
                    33% {{ border-color: #10b981; }}
                    66% {{ border-color: #3b82f6; }}
                    100% {{ border-color: #ef4444; }}
                }}
                .count-display {{
                    animation: pulse 0.5s ease-in-out, slideIn 0.3s ease-out;
                }}
                .container-animated {{
                    animation: rainbow 3s infinite;
                }}
            </style>
            <div style="text-align: center; padding: 40px; font-family: system-ui;
                        background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);">
                <div class="count-display" style="font-size: 90px; margin: 20px 0;
                     color: {color}; font-weight: 900; text-shadow: 2px 2px 4px rgba(0,0,0,0.2);">
                    {count} ✨
                </div>
                <p style="font-size: 20px; color: #92400e; margin: 10px 0; font-weight: 700;">
                    🎨 Dynamic Counter with Animations!
                </p>
                <div style="margin-top: 30px;">
                    <button onclick="counter.decrement()"
                            style="padding: 16px 32px; font-size: 20px; margin: 0 10px;
                                   background: linear-gradient(135deg, #f87171 0%, #dc2626 100%);
                                   color: white; border: none; border-radius: 12px;
                                   cursor: pointer; font-weight: 700;
                                   box-shadow: 0 6px 12px rgba(239,68,68,0.4);
                                   transition: transform 0.2s, box-shadow 0.2s;"
                            onmouseover="this.style.transform='translateY(-3px) scale(1.05)';
                                       this.style.boxShadow='0 8px 16px rgba(239,68,68,0.5)'"
                            onmouseout="this.style.transform='translateY(0) scale(1)';
                                      this.style.boxShadow='0 6px 12px rgba(239,68,68,0.4)'">
                        ⬇️ Down
                    </button>
                    <button onclick="counter.reset()"
                            style="padding: 16px 32px; font-size: 20px; margin: 0 10px;
                                   background: linear-gradient(135deg, #a78bfa 0%, #7c3aed 100%);
                                   color: white; border: none; border-radius: 12px;
                                   cursor: pointer; font-weight: 700;
                                   box-shadow: 0 6px 12px rgba(124,58,237,0.4);
                                   transition: transform 0.2s, box-shadow 0.2s;"
                            onmouseover="this.style.transform='translateY(-3px) scale(1.05)';
                                       this.style.boxShadow='0 8px 16px rgba(124,58,237,0.5)'"
                            onmouseout="this.style.transform='translateY(0) scale(1)';
                                      this.style.boxShadow='0 6px 12px rgba(124,58,237,0.4)'">
                        🔄 Zero
                    </button>
                    <button onclick="counter.increment()"
                            style="padding: 16px 32px; font-size: 20px; margin: 0 10px;
                                   background: linear-gradient(135deg, #34d399 0%, #059669 100%);
                                   color: white; border: none; border-radius: 12px;
                                   cursor: pointer; font-weight: 700;
                                   box-shadow: 0 6px 12px rgba(16,185,129,0.4);
                                   transition: transform 0.2s, box-shadow 0.2s;"
                            onmouseover="this.style.transform='translateY(-3px) scale(1.05)';
                                       this.style.boxShadow='0 8px 16px rgba(16,185,129,0.5)'"
                            onmouseout="this.style.transform='translateY(0) scale(1)';
                                      this.style.boxShadow='0 6px 12px rgba(16,185,129,0.4)'">
                        ⬆️ Up
                    </button>
                </div>
                <div class="container-animated"
                     style="margin-top: 40px; padding: 28px; background: white;
                            border-radius: 16px; border: 4px solid #ef4444;
                            box-shadow: 0 12px 40px rgba(0,0,0,0.15);">
                    <p style="margin: 0; color: #7c2d12; font-weight: 800; font-size: 20px;">
                        🧬 Morpheus Counter - Version 3 (Animated!)
                    </p>
                    <p style="margin: 14px 0 0 0; color: #92400e; font-size: 16px;">
                        Watch the colors change based on the count!
                    </p>
                    <p style="margin: 10px 0 0 0; color: #92400e; font-size: 14px;">
                        • Positive = Green 💚
                    </p>
                    <p style="margin: 4px 0 0 0; color: #92400e; font-size: 14px;">
                        • Zero = Blue 💙
                    </p>
                    <p style="margin: 4px 0 0 0; color: #92400e; font-size: 14px;">
                        • Negative = Red ❤️
                    </p>
                    <p style="margin: 12px 0 0 0; color: #6b7280; font-size: 13px; font-style: italic;">
                        AI added animations and dynamic styling! 🎭
                    </p>
                </div>
            </div>
            "#,
            count = self.count,
            color = text_color,
        ));
    }
}

/// Initialize the counter (called from JavaScript)
#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"🧬 Morpheus Counter Component V3 (Animated) loaded!".into());
}
