use anathema_store::smallmap::SmallMap;
use anathema_store::storage::strings::StringId;

use super::ComponentBlueprintId;
use super::components::AssocEventMapping;
use super::expressions::ExpressionId;

/// Named widget node.
#[derive(Debug, Clone, PartialEq)]
pub struct Single {
    pub ident: String,
    pub children: Vec<Blueprint>,
    pub attributes: SmallMap<String, ExpressionId>,
    pub value: Option<ExpressionId>,
}

/// For-loop node.
#[derive(Debug, Clone, PartialEq)]
pub struct For {
    pub binding: String,
    pub data: ExpressionId,
    pub body: Vec<Blueprint>,
}

/// With-binding node: `with expr as name`.
#[derive(Debug, Clone, PartialEq)]
pub struct With {
    pub binding: String,
    pub data: ExpressionId,
    pub body: Vec<Blueprint>,
}

/// If/else control flow.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlFlow {
    pub elses: Vec<Else>,
}

/// Else branch with optional condition.
#[derive(Debug, Clone, PartialEq)]
pub struct Else {
    pub cond: Option<ExpressionId>,
    pub body: Vec<Blueprint>,
}

/// Component node.
#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub name: String,
    pub name_id: StringId,
    pub id: ComponentBlueprintId,
    pub body: Vec<Blueprint>,
    pub attributes: SmallMap<String, ExpressionId>,
    pub assoc_functions: Vec<AssocEventMapping>,
    /// Parent component ID.
    pub parent: Option<ComponentBlueprintId>,
}

/// Compiled template structure.
#[derive(Clone, Debug, PartialEq)]
pub enum Blueprint {
    /// Named widget.
    Single(Single),
    /// For-loop.
    For(For),
    /// With-binding.
    With(With),
    /// If/else.
    ControlFlow(ControlFlow),
    /// Component.
    Component(Component),
    /// Component slot.
    Slot(Vec<Self>),
}

macro_rules! single {
    ($ident:expr) => {
        $crate::templates::blueprints::Blueprint::Single(Single {
            ident: $ident.into(),
            children: vec![],
            attributes: SmallMap::empty(),
            value: None,
        })
    };
    (value @ $ident:expr, $value:expr) => {
        $crate::templates::blueprints::Blueprint::Single(Single {
            ident: $ident.into(),
            children: vec![],
            attributes: SmallMap::empty(),
            value: Some($value.into()),
        })
    };
    (children @ $ident:expr, $children:expr) => {
        $crate::templates::blueprints::Blueprint::Single(Single {
            ident: $ident.into(),
            children: $children,
            attributes: SmallMap::empty(),
            value: None,
        })
    };
}

macro_rules! forloop {
    ($binding:expr, $data:expr, $body:expr) => {
        $crate::templates::blueprints::Blueprint::For(For {
            binding: $binding.into(),
            data: $data,
            body: $body,
        })
    };
}

pub(crate) use {forloop, single};
