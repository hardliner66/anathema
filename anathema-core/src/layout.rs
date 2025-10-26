//! Layout storage.
//!
//! Stores position and size for each element.
//!
//! Coordinates: origin (0, 0) at top-left, X right, Y down.

use anathema_geometry::{Pos, Region, Size};
use anathema_store::slab::SecondaryMap;

use crate::runtime::elements::ElementId;

/// Maps element IDs to regions (position + size).
#[derive(Debug)]
pub struct Layout {
    regions: SecondaryMap<ElementId, Region>,
}

impl Layout {
    /// Create empty layout storage.
    pub(crate) fn empty() -> Self {
        Self {
            regions: SecondaryMap::empty(),
        }
    }

    /// Set element size.
    ///
    /// # Panics
    ///
    /// Panics if element not inserted.
    pub(crate) fn set_size(&mut self, id: ElementId, size: Size) {
        self.regions[id].resize(size);
    }

    /// Set element position.
    ///
    /// # Panics
    ///
    /// Panics if element not inserted.
    pub(crate) fn set_pos(&mut self, id: ElementId, pos: Pos) {
        self.regions[id].set_pos(pos);
    }

    /// Insert element with zero region.
    pub(crate) fn insert(&mut self, id: ElementId) {
        self.regions.insert(id, Region::ZERO);
    }

    /// Iterate regions.
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter()
    }
}
