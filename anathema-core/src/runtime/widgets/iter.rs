use std::cell::RefMut;

use anathema_geometry::Size;

use crate::layout::Layout;
use crate::runtime::elements::{Element, ElementId, Elements};
use crate::runtime::widgets::Widget;

/// Iterator over widget children.
///
/// # Invariant
///
/// All element IDs must reference `Widget` elements, not other element types.
pub struct Children<'a, 'bp> {
    children: &'a [ElementId],
    elements: &'a Elements<'bp>,
    index: usize,
}

impl<'a, 'bp> Iterator for Children<'a, 'bp> {
    type Item = WidgetRef<'a, 'bp>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.children.len() {
            return None;
        }

        let id = self.children[self.index];
        self.index += 1;

        let node = &self.elements[id];
        let Element::Widget(widget) = &node.element else { unreachable!() };

        let children = Children {
            children: &node.children,
            elements: self.elements,
            index: 0,
        };

        let widget_ref = WidgetRef {
            id,
            widget: widget.borrow_mut(),
            children,
        };

        Some(widget_ref)
    }
}

/// Reference to a widget with its children.
pub struct WidgetRef<'a, 'bp> {
    id: ElementId,
    widget: RefMut<'a, Box<dyn Widget>>,
    children: Children<'a, 'bp>,
}

impl<'a, 'bp> WidgetRef<'a, 'bp> {
    /// Calculate widget layout and return its size.
    pub fn layout(mut self, layout: &mut Layout) -> Size {
        self.widget.layout(self.children, layout)
    }
}
