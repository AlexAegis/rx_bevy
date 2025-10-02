use crate::SignalContext;

pub trait FromContext: SignalContext {
	fn from_context(context: &mut Self::Context) -> Self;
}
