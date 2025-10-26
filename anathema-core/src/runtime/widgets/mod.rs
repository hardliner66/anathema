//! Widget trait and registration.
//!
//! Renderable UI elements. Register by name, templates instantiate them.
//!
//! [`Widget`] methods:
//! - `layout`: Calculate size from constraints and children
//! - `position`: Return position
//! - `paint`: Render to screen
//! - `describe`: Debug name (optional)
//!
//! # Example
//!
//! ```rust,ignore
//! use anathema_core::runtime::{Widget, Children};
//! use anathema_core::layout::Layout;
//! use anathema_geometry::{Pos, Size};
//!
//! struct VStack {
//!     pos: Pos,
//! }
//!
//! impl Widget for VStack {
//!     fn layout(&mut self, mut children: Children<'_, '_>, layout: &mut Layout) -> Size {
//!         let mut height = 0;
//!         let mut width = 0;
//!         for child in children {
//!             let size = child.layout(layout);
//!             height += size.height;
//!             width = width.max(size.width);
//!         }
//!         Size::new(width, height)
//!     }
//!
//!     fn position(&mut self) -> Pos {
//!         self.pos
//!     }
//!
//!     fn paint(&mut self) {}
//! }
//! ```

use std::collections::HashMap;

use anathema_geometry::{Pos, Size};
use anathema_store::slab::{GenSlab, Key, SecondaryMap};

use crate::attributes::Attributes;
use crate::layout::Layout;
use crate::runtime::elements::ElementId;
use crate::runtime::widgets::iter::Children;

type WidgetFactory = Box<dyn Fn(&Attributes<'_>) -> Box<dyn Widget>>;

pub mod iter;

/// Widget registry.
#[derive(Default)]
pub struct RegisteredWidgets {
    registry: HashMap<Box<str>, WidgetFactory>,
}

impl std::fmt::Debug for RegisteredWidgets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.registry.keys()).finish()
    }
}

impl RegisteredWidgets {
    /// Create an empty registry.
    pub fn empty() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    /// Register a widget type implementing `Default`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// widgets.register_default::<Text>("text");
    /// ```
    pub fn register_default<T: Widget + Default>(&mut self, ident: impl Into<Box<str>>) {
        self.registry
            .insert(ident.into(), Box::new(|_attr| Box::<T>::default()));
    }

    /// Create widget instance by name.
    ///
    /// Returns `Err(())` if widget name not found.
    pub fn make(&self, ident: &str, attributes: &Attributes<'_>) -> Result<Box<dyn Widget>, ()> {
        let Some(factory) = self.registry.get(ident) else { return Err(()) };
        let element = factory(attributes);
        Ok(element)
    }
}

/// Renderable UI element.
pub trait Widget: 'static {
    /// Calculate size from constraints and children.
    fn layout(&mut self, children: Children<'_, '_>, layout: &mut Layout) -> Size;

    /// Return position.
    fn position(&mut self) -> Pos;

    /// Render to screen.
    fn paint(&mut self);

    /// Debug description.
    fn describe(&self) -> &str {
        "<dyn Element>"
    }
}

impl std::fmt::Debug for dyn Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe())
    }
}

/// Node in the widget tree.
pub struct Node {
    // TODO: do we need the id here?
    id: ElementId,
    children: Vec<ElementId>,
}

impl Node {
    pub fn new(id: ElementId, children: Vec<ElementId>) -> Self {
        Self { id, children }
    }
}

/// Widget tree.
pub struct Widgets {
    pub widgets: SecondaryMap<ElementId, Node>,
}

impl Widgets {
    pub(crate) fn empty() -> Self {
        Self {
            widgets: SecondaryMap::empty(),
        }
    }
}
