use bevy_ecs::system::Commands;
use rx_bevy_core::{DropUnsafeSignalContext, SignalContext};

/// A context that offers a mutable commands reference, it is always unsafe, but managed.
/// This can be used to extend a context in case a custom subscriber would need something
/// more in it's context.
pub trait ContextWithCommands<'a>: SignalContext<DropSafety = DropUnsafeSignalContext> {
	fn commands(&mut self) -> &mut Commands<'a, 'a>;
}
