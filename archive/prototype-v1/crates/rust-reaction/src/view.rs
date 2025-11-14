//! View construction using the builder pattern.
//!
//! This module provides a Rust-native approach to building DOM trees
//! using method chaining instead of JSX-like macros.

use wasm_bindgen::JsCast;
use web_sys::{self, HtmlElement};

/// A view that can be rendered to the DOM.
pub trait View {
    /// Render this view to a DOM element.
    fn render(&self) -> web_sys::Element;

    /// Update the DOM element with changes.
    fn update(&self, element: &web_sys::Element);
}

/// A text node view.
pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl View for Text {
    fn render(&self) -> web_sys::Element {
        let window = web_sys::window().expect("no window");
        let document = window.document().expect("no document");
        let text_node = document.create_text_node(&self.content);
        let span = document.create_element("span").expect("failed to create span");
        span.append_child(&text_node).expect("failed to append text");
        span
    }

    fn update(&self, element: &web_sys::Element) {
        element.set_text_content(Some(&self.content));
    }
}

/// Convenience function for creating text views.
pub fn text(content: impl Into<String>) -> Text {
    Text::new(content)
}

/// A marker trait for HTML elements that can have a class attribute.
pub trait HasClass: Sized {
    fn class(self, class: impl Into<String>) -> Self;
}

/// A marker trait for HTML elements that can have an href attribute.
pub trait HasHref: Sized {
    fn href(self, href: impl Into<String>) -> Self;
}

/// A marker trait for HTML elements that can have children.
pub trait HasChildren: Sized {
    fn child(self, child: impl View + 'static) -> Self;
    fn children_from_iter<I>(self, children: I) -> Self
    where
        I: IntoIterator<Item = impl View + 'static>;
}

/// A generic HTML element builder.
pub struct Element<T = HtmlElement> {
    tag: String,
    classes: Vec<String>,
    attributes: Vec<(String, String)>,
    children: Vec<Box<dyn View>>,
    event_handlers: Vec<Box<dyn Fn(web_sys::Event)>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Element<T> {
    fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            classes: Vec::new(),
            attributes: Vec::new(),
            children: Vec::new(),
            event_handlers: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push((name.into(), value.into()));
        self
    }

    pub fn text(mut self, content: impl Into<String>) -> Self {
        self.children.push(Box::new(Text::new(content)));
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(web_sys::MouseEvent) + 'static,
    {
        let callback = move |event: web_sys::Event| {
            if let Some(mouse_event) = event.dyn_ref::<web_sys::MouseEvent>() {
                handler(mouse_event.clone());
            }
        };
        self.event_handlers.push(Box::new(callback));
        self
    }
}

impl<T> HasClass for Element<T> {
    fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }
}

impl<T> HasChildren for Element<T> {
    fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    fn children_from_iter<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = impl View + 'static>,
    {
        for child in children {
            self.children.push(Box::new(child));
        }
        self
    }
}

impl<T> View for Element<T> {
    fn render(&self) -> web_sys::Element {
        let window = web_sys::window().expect("no window");
        let document = window.document().expect("no document");
        let element = document
            .create_element(&self.tag)
            .expect("failed to create element");

        // Set classes
        if !self.classes.is_empty() {
            element
                .set_attribute("class", &self.classes.join(" "))
                .expect("failed to set class");
        }

        // Set attributes
        for (name, value) in &self.attributes {
            element
                .set_attribute(name, value)
                .expect("failed to set attribute");
        }

        // Append children
        for child in &self.children {
            let child_element = child.render();
            element
                .append_child(&child_element)
                .expect("failed to append child");
        }

        element
    }

    fn update(&self, element: &web_sys::Element) {
        // Update classes
        if !self.classes.is_empty() {
            element
                .set_attribute("class", &self.classes.join(" "))
                .expect("failed to set class");
        }

        // Update attributes
        for (name, value) in &self.attributes {
            element
                .set_attribute(name, value)
                .expect("failed to set attribute");
        }

        // TODO: Reconcile children efficiently
    }
}

// Common HTML elements

/// Create a div element.
pub fn div() -> Element<web_sys::HtmlDivElement> {
    Element::new("div")
}

/// Create a paragraph element.
pub fn p() -> Element<web_sys::HtmlParagraphElement> {
    Element::new("p")
}

/// Create a button element.
pub fn button() -> Element<web_sys::HtmlButtonElement> {
    Element::new("button")
}

/// Create a span element.
pub fn span() -> Element<web_sys::HtmlSpanElement> {
    Element::new("span")
}

/// Create an anchor element.
pub fn a() -> Element<web_sys::HtmlAnchorElement> {
    Element::new("a")
}

impl HasHref for Element<web_sys::HtmlAnchorElement> {
    fn href(self, href: impl Into<String>) -> Self {
        self.attr("href", href)
    }
}

/// Create an unordered list element.
pub fn ul() -> Element<web_sys::HtmlUListElement> {
    Element::new("ul")
}

/// Create a list item element.
pub fn li() -> Element<web_sys::HtmlLiElement> {
    Element::new("li")
}

/// Create an input element.
pub fn input() -> Element<web_sys::HtmlInputElement> {
    Element::new("input")
}
