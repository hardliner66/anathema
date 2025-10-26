//! Runtime execution of compiled templates.
//!
//! # Components
//!
//! Reusable UI elements with state and message handling.
//!
//! - Single instance: Shared across all template references
//! - Prototype: New instance per template reference
//!
//! # Widgets
//!
//! Renderable primitives (`text`, `border`, `vstack`, etc.).
//! Register by name via [`RegisteredWidgets`].
//!
//! [`Widget`] trait methods:
//! - `layout`: Calculate size from constraints and children
//! - `position`: Return current position
//! - `paint`: Render to screen
//!
//! # Template Values
//!
//! [`TemplateValue`] represents runtime values from expression evaluation.
//! Supports primitives, strings, colors, collections, and dynamic state values.

pub use widgets::iter::Children;
pub use widgets::{RegisteredWidgets, Widget};

pub use crate::runtime::eval::values::TemplateValue;

pub mod components;
pub(crate) mod elements;
mod error;
pub(crate) mod eval;
pub(crate) mod functions;
pub(crate) mod widgets;
