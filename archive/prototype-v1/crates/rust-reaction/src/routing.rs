//! Type-safe routing using enums.
//!
//! This module provides a Rust-native approach to routing using enums
//! and pattern matching rather than string-based routes.

use std::fmt;

/// A route that can be converted to and from a path.
pub trait Route: Sized {
    /// Convert this route to a URL path.
    fn to_path(&self) -> String;

    /// Try to parse a URL path into this route.
    fn from_path(path: &str) -> Result<Self, RouteError>;
}

/// Errors that can occur during routing.
#[derive(Debug, Clone)]
pub enum RouteError {
    /// The path didn't match any known route.
    NotFound(String),
    /// The path matched a route but had invalid parameters.
    InvalidParameter {
        route: String,
        parameter: String,
        error: String,
    },
}

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RouteError::NotFound(path) => write!(f, "Route not found: {}", path),
            RouteError::InvalidParameter {
                route,
                parameter,
                error,
            } => write!(
                f,
                "Invalid parameter '{}' for route '{}': {}",
                parameter, route, error
            ),
        }
    }
}

impl std::error::Error for RouteError {}

/// A router that manages navigation between routes.
pub struct Router<R: Route> {
    current_route: R,
    history: Vec<R>,
}

impl<R: Route> Router<R> {
    /// Create a new router with an initial route.
    pub fn new(initial_route: R) -> Self {
        Self {
            current_route: initial_route,
            history: Vec::new(),
        }
    }

    /// Get the current route.
    pub fn current(&self) -> &R {
        &self.current_route
    }

    /// Navigate to a new route.
    pub fn navigate(&mut self, route: R) {
        let old_route = std::mem::replace(&mut self.current_route, route);
        self.history.push(old_route);
    }

    /// Go back to the previous route, if any.
    pub fn go_back(&mut self) -> Option<()> {
        let previous = self.history.pop()?;
        self.current_route = previous;
        Some(())
    }

    /// Check if we can go back.
    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }
}

/// Helper macro for deriving Route implementations on enums.
///
/// Example:
/// ```ignore
/// #[derive(Debug, Clone, Route)]
/// enum AppRoute {
///     #[route("/")]
///     Home,
///     #[route("/about")]
///     About,
///     #[route("/user/:id")]
///     User { id: u32 },
/// }
/// ```
