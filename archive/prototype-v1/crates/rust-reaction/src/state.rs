//! State management primitives.
//!
//! This module provides Rust-native state management using explicit ownership
//! and the observer pattern rather than implicit reactivity tracking.

use std::cell::RefCell;
use std::rc::Rc;

/// A container for observable state.
pub struct State<T> {
    value: T,
    observers: Vec<Rc<dyn Fn(&T)>>,
}

impl<T> State<T> {
    /// Create a new state with an initial value.
    pub fn new(value: T) -> Self {
        Self {
            value,
            observers: Vec::new(),
        }
    }

    /// Get a reference to the current value.
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Update the state value and notify observers.
    pub fn update(&mut self, f: impl FnOnce(&mut T)) {
        f(&mut self.value);
        self.notify();
    }

    /// Set the state to a new value and notify observers.
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.notify();
    }

    /// Map the current value to a new value using a function.
    pub fn map<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        f(&self.value)
    }

    /// Add an observer that will be called when the state changes.
    pub fn observe(&mut self, observer: impl Fn(&T) + 'static) {
        self.observers.push(Rc::new(observer));
    }

    /// Notify all observers of the current state.
    fn notify(&self) {
        for observer in &self.observers {
            observer(&self.value);
        }
    }
}

/// A shared state that can be cloned and shared across components.
pub struct SharedState<T> {
    inner: Rc<RefCell<State<T>>>,
}

impl<T> SharedState<T> {
    /// Create a new shared state with an initial value.
    pub fn new(value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(State::new(value))),
        }
    }

    /// Get a reference to the current value.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let state = self.inner.borrow();
        f(state.get())
    }

    /// Update the state value and notify observers.
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        self.inner.borrow_mut().update(f);
    }

    /// Set the state to a new value and notify observers.
    pub fn set(&self, value: T) {
        self.inner.borrow_mut().set(value);
    }

    /// Add an observer that will be called when the state changes.
    pub fn observe(&self, observer: impl Fn(&T) + 'static) {
        self.inner.borrow_mut().observe(observer);
    }
}

impl<T> Clone for SharedState<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

/// A lens for focusing on a part of a larger state.
pub trait Lens<T, U> {
    /// View a part of the source value.
    fn view(&self, source: &T) -> U;

    /// Update the source value through the lens.
    fn update(&self, source: &mut T, value: U);
}

/// Create a derived state from an existing state using a lens.
pub struct Derived<T, U, L> {
    source: SharedState<T>,
    lens: L,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U, L> Derived<T, U, L>
where
    L: Lens<T, U>,
{
    /// Create a new derived state.
    pub fn new(source: SharedState<T>, lens: L) -> Self {
        Self {
            source,
            lens,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get the derived value.
    pub fn get(&self) -> U {
        self.source.with(|src| self.lens.view(src))
    }

    /// Update the source through the lens.
    pub fn set(&self, value: U) {
        self.source.update(|src| self.lens.update(src, value));
    }
}
