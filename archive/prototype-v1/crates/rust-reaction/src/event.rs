//! Event handling utilities.
//!
//! This module provides Rust-native event handling using ownership
//! rather than requiring manual cloning for callbacks.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

/// An event listener that automatically removes itself when dropped (RAII).
pub struct EventListener {
    target: web_sys::EventTarget,
    event_type: String,
    closure: Option<Closure<dyn FnMut(web_sys::Event)>>,
}

impl EventListener {
    /// Create a new event listener.
    pub fn new<F>(
        target: &web_sys::EventTarget,
        event_type: impl Into<String>,
        mut callback: F,
    ) -> Self
    where
        F: FnMut(web_sys::Event) + 'static,
    {
        let event_type = event_type.into();
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            callback(event);
        }) as Box<dyn FnMut(web_sys::Event)>);

        target
            .add_event_listener_with_callback(&event_type, closure.as_ref().unchecked_ref())
            .expect("failed to add event listener");

        Self {
            target: target.clone(),
            event_type,
            closure: Some(closure),
        }
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        if let Some(closure) = self.closure.take() {
            self.target
                .remove_event_listener_with_callback(
                    &self.event_type,
                    closure.as_ref().unchecked_ref(),
                )
                .expect("failed to remove event listener");
        }
    }
}

/// An effect that uses RAII for automatic cleanup.
pub trait Effect {
    /// The output type of this effect (e.g., an event listener).
    type Output;

    /// Run the effect, returning a handle that will clean up when dropped.
    fn run(&self) -> Self::Output;
}

/// A struct that represents running an effect with automatic cleanup.
pub struct EffectHandle<E: Effect> {
    _output: E::Output,
}

impl<E: Effect> EffectHandle<E> {
    /// Create a new effect handle by running the effect.
    pub fn new(effect: E) -> Self {
        Self {
            _output: effect.run(),
        }
    }
}
