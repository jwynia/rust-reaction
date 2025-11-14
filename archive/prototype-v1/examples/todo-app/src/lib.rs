//! Todo App Example
//!
//! This example demonstrates a more complex application with multiple components,
//! state management, and user input handling.

use rust_reaction::prelude::*;
use wasm_bindgen::prelude::*;

/// A single todo item.
#[derive(Debug, Clone)]
pub struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

impl Todo {
    fn new(id: usize, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }

    fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}

/// The main todo application component.
pub struct TodoApp {
    todos: Vec<Todo>,
    next_id: usize,
    current_input: String,
}

/// Messages for the todo application.
pub enum TodoMsg {
    AddTodo,
    ToggleTodo(usize),
    DeleteTodo(usize),
    UpdateInput(String),
    ClearCompleted,
}

impl TodoApp {
    pub fn new() -> Self {
        Self {
            todos: vec![
                Todo::new(0, "Learn Rust".into()),
                Todo::new(1, "Build a web app".into()),
                Todo::new(2, "Deploy to production".into()),
            ],
            next_id: 3,
            current_input: String::new(),
        }
    }

    fn add_todo(&mut self) {
        if !self.current_input.trim().is_empty() {
            let todo = Todo::new(self.next_id, self.current_input.clone());
            self.todos.push(todo);
            self.next_id += 1;
            self.current_input.clear();
        }
    }

    fn toggle_todo(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.toggle();
        }
    }

    fn delete_todo(&mut self, id: usize) {
        self.todos.retain(|t| t.id != id);
    }

    fn clear_completed(&mut self) {
        self.todos.retain(|t| !t.completed);
    }

    fn stats(&self) -> (usize, usize, usize) {
        let total = self.todos.len();
        let completed = self.todos.iter().filter(|t| t.completed).count();
        let active = total - completed;
        (total, active, completed)
    }
}

impl Component for TodoApp {
    type Message = TodoMsg;

    fn view(&self) -> impl View {
        let (total, active, completed) = self.stats();

        div()
            .class("todo-app")
            .child(
                div()
                    .class("todo-header")
                    .child(
                        text("Rust Reaction Todo App")
                    )
            )
            .child(
                div()
                    .class("todo-input")
                    .child(
                        input()
                            .attr("type", "text")
                            .attr("placeholder", "What needs to be done?")
                            .attr("value", &self.current_input)
                    )
                    .child(
                        button()
                            .class("btn-add")
                            .text("Add")
                    )
            )
            .child(
                div()
                    .class("todo-list")
                    .children_from_iter(
                        self.todos.iter().map(|todo| self.render_todo(todo))
                    )
            )
            .child(
                div()
                    .class("todo-footer")
                    .child(
                        span()
                            .class("todo-stats")
                            .text(format!("{} total, {} active, {} completed", total, active, completed))
                    )
                    .child(
                        button()
                            .class("btn-clear")
                            .text("Clear Completed")
                    )
            )
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            TodoMsg::AddTodo => self.add_todo(),
            TodoMsg::ToggleTodo(id) => self.toggle_todo(id),
            TodoMsg::DeleteTodo(id) => self.delete_todo(id),
            TodoMsg::UpdateInput(text) => self.current_input = text,
            TodoMsg::ClearCompleted => self.clear_completed(),
        }
    }
}

impl TodoApp {
    fn render_todo(&self, todo: &Todo) -> impl View + '_ {
        let item_class = if todo.completed {
            "todo-item completed"
        } else {
            "todo-item"
        };

        div()
            .class(item_class)
            .child(
                input()
                    .attr("type", "checkbox")
                    .attr("checked", if todo.completed { "checked" } else { "" })
            )
            .child(
                span()
                    .class("todo-text")
                    .text(&todo.text)
            )
            .child(
                button()
                    .class("btn-delete")
                    .text("×")
            )
    }
}

/// Entry point for the WASM module.
#[wasm_bindgen(start)]
pub fn run() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let app = TodoApp::new();
    mount_to_body(app);
}
