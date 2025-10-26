/// Associates a component with its state and message types.
///
/// # Example
///
/// ```rust,ignore
/// use anathema_core::runtime::components::Component;
///
/// struct Counter;
///
/// impl Component for Counter {
///     type State = CounterState;
///     type Message = CounterMessage;
/// }
/// ```
pub trait Component: 'static {
    /// State type. Must implement `anathema_state::State`.
    type State;

    /// Message type for events.
    type Message;
}

/// Type-erased component for runtime polymorphism.
///
/// Auto-implemented for all `Component` types.
pub trait AnyComponent: 'static {}

impl std::fmt::Debug for dyn AnyComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<component>")
    }
}

impl<T: Component> AnyComponent for T {}
