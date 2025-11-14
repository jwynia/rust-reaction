//! DOM utilities and helpers.

use web_sys;

/// Get the window object.
pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global window exists")
}

/// Get the document object.
pub fn document() -> web_sys::Document {
    window().document().expect("no document exists")
}

/// Get an element by its ID.
pub fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
    document().get_element_by_id(id)
}

/// Get the body element.
pub fn body() -> web_sys::HtmlElement {
    document().body().expect("no body element")
}

/// Mount a component to an element with the given ID.
pub fn mount_to_id<C>(component: C, id: &str) -> crate::component::ComponentHandle<C>
where
    C: crate::component::Component,
{
    let container = get_element_by_id(id)
        .unwrap_or_else(|| panic!("element with id '{}' not found", id));
    crate::component::ComponentHandle::mount(component, &container)
}

/// Mount a component to the body.
pub fn mount_to_body<C>(component: C) -> crate::component::ComponentHandle<C>
where
    C: crate::component::Component,
{
    crate::component::ComponentHandle::mount(component, &body())
}
