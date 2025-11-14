//! Component abstraction for building reusable UI elements.
//!
//! Components in Rust Reaction are structs that implement the `Component` trait,
//! embracing Rust's ownership model rather than using function components with hooks.

use crate::view::View;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys;

/// A component that can render itself and handle messages.
pub trait Component: Sized + 'static {
    /// The type of messages this component can handle.
    type Message;

    /// Render the component's current state to a view.
    fn view(&self) -> impl View;

    /// Update the component's state in response to a message.
    fn update(&mut self, msg: Self::Message);

    /// Called when the component is first mounted.
    fn mounted(&mut self) {}

    /// Called when the component is about to be unmounted.
    fn unmounted(&mut self) {}
}

/// A handle to a mounted component instance.
pub struct ComponentHandle<C: Component> {
    component: Rc<RefCell<C>>,
    root_element: web_sys::Element,
}

impl<C: Component> ComponentHandle<C> {
    /// Create a new component handle and mount it to the DOM.
    pub fn mount(component: C, container: &web_sys::Element) -> Self {
        let root_element = component.view().render();
        container
            .append_child(&root_element)
            .expect("failed to mount component");

        let handle = Self {
            component: Rc::new(RefCell::new(component)),
            root_element,
        };

        handle.component.borrow_mut().mounted();
        handle
    }

    /// Send a message to the component.
    pub fn send(&self, msg: C::Message) {
        self.component.borrow_mut().update(msg);
        self.re_render();
    }

    /// Get a reference to the underlying component.
    pub fn component(&self) -> std::cell::Ref<C> {
        self.component.borrow()
    }

    /// Get a mutable reference to the underlying component.
    pub fn component_mut(&self) -> std::cell::RefMut<C> {
        self.component.borrow_mut()
    }

    /// Re-render the component.
    fn re_render(&self) {
        let component = self.component.borrow();
        let new_view = component.view();
        new_view.update(&self.root_element);
    }

    /// Unmount the component from the DOM.
    pub fn unmount(self) {
        self.component.borrow_mut().unmounted();
        self.root_element
            .parent_node()
            .expect("no parent")
            .remove_child(&self.root_element)
            .expect("failed to remove element");
    }
}

impl<C: Component> Clone for ComponentHandle<C> {
    fn clone(&self) -> Self {
        Self {
            component: Rc::clone(&self.component),
            root_element: self.root_element.clone(),
        }
    }
}

/// A component with no messages (stateless).
pub enum Never {}

/// Helper macro for implementing simple components.
#[macro_export]
macro_rules! stateless_component {
    ($name:ident) => {
        impl Component for $name {
            type Message = Never;

            fn update(&mut self, _msg: Self::Message) {
                match _msg {}
            }
        }
    };
}
