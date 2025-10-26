//! Component registration and lifecycle.
//!
//! # Component Types
//!
//! - **Single instance**: Shared across all template references
//! - **Prototype**: New instance per reference
//!
//! # Usage
//!
//! ```rust,ignore
//! // Single instance
//! components.insert_component(blueprint_id, MyComponent, MyState::default());
//!
//! // Prototype
//! components.insert_prototype(
//!     blueprint_id,
//!     Box::new(|| Box::new(MyComponent)),
//!     Box::new(|| Box::new(MyState::default())),
//! );
//! ```

use anathema::Value;
use anathema_state::State;
use anathema_store::key;
use anathema_store::slab::{GenSlab, SecondaryMap};

pub use self::component::{AnyComponent, Component};
use crate::templates::ComponentBlueprintId;

key!(ComponentId, Debug, Copy, Clone);

/// Factory that creates component instances.
pub(crate) type FnComp = Box<dyn Fn() -> Box<dyn AnyComponent>>;

/// Factory that creates state instances.
pub(crate) type FnState = Box<dyn Fn() -> Box<dyn State>>;

mod component;

struct Entry {
    component: Box<dyn AnyComponent>,
    state: Value<Box<dyn State>>,
    kind: ComponentKind,
}

enum ComponentKind {
    Component,
    PrototypeInstance,
}

enum Lookup {
    Prototype(FnComp, FnState),
    Component(ComponentId),
}

/// Component registry.
///
/// Maps blueprint IDs to instances or factories.
pub struct Components {
    instances: GenSlab<ComponentId, Entry>,
    blueprints: SecondaryMap<ComponentBlueprintId, Lookup>,
}

impl Components {
    /// Create an empty component registry.
    pub fn empty() -> Self {
        Self {
            instances: GenSlab::empty(),
            blueprints: SecondaryMap::empty(),
        }
    }

    /// Register single-instance component.
    pub(crate) fn insert_component(
        &mut self,
        blueprint_id: ComponentBlueprintId,
        component: impl AnyComponent,
        state: impl State,
    ) -> ComponentId {
        let entry = Entry {
            component: Box::new(component),
            state: Value::new(Box::new(state)),
            kind: ComponentKind::Component,
        };
        let component_id = self.instances.insert(entry);
        self.blueprints.insert(blueprint_id, Lookup::Component(component_id));
        component_id
    }

    /// Register prototype with factories.
    pub(crate) fn insert_prototype(&mut self, blueprint_id: ComponentBlueprintId, component: FnComp, state: FnState) {
        self.blueprints
            .insert(blueprint_id, Lookup::Prototype(component, state));
    }

    /// Get or create instance by blueprint ID.
    ///
    /// # Panics
    ///
    /// Panics if blueprint ID not found.
    pub(crate) fn by_blueprint_id(&mut self, id: ComponentBlueprintId) -> ComponentId {
        match self.blueprints.get(id) {
            Some(Lookup::Component(id)) => *id,
            Some(Lookup::Prototype(comp, state)) => {
                let entry = Entry {
                    component: comp(),
                    state: Value::new(state()),
                    kind: ComponentKind::PrototypeInstance,
                };
                self.instances.insert(entry)
            }
            None => todo!(),
        }
    }

    pub(crate) fn by_component_id(&self) {}

    /// Get component state.
    pub(crate) fn get_state(&self, component_id: ComponentId) -> Option<&Value<Box<dyn State>>> {
        let inst = self.instances.get(component_id)?;
        Some(&inst.state)
    }

    /// Get mutable reference to component state.
    pub(crate) fn get_state_mut(&mut self, component_id: ComponentId) -> Option<&mut Value<Box<dyn State>>> {
        let inst = self.instances.get_mut(component_id)?;
        Some(&mut inst.state)
    }
}

impl Default for Components {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::fmt::Debug for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<components>")
    }
}
