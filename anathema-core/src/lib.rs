//! Core runtime and template system for Anathema.
//!
//! Provides:
//! - Template compilation to blueprints
//! - Runtime execution
//! - Widget trait and registration
//! - Component management
//! - Layout storage
//!
//! # Template Compilation
//!
//! ```rust,ignore
//! use anathema_core::templates::{Document, Variables};
//!
//! let mut doc = Document::new("text 'Hello'");
//! let mut vars = Variables::new();
//! let blueprint = doc.compile(&mut vars)?;
//! ```
//!
//! # Components
//!
//! Stateful UI elements with templates. Implement [`Component`](runtime::components::Component)
//! to define state and message types.
//!
//! # Widgets
//!
//! Render UI elements. Implement [`Widget`](runtime::Widget) for layout and painting.

#[allow(unused_extern_crates)]
extern crate anathema_state as anathema;

pub mod attributes;
pub mod frontend;
pub mod layout;
pub mod runtime;
pub mod templates;

pub(crate) mod testing;
