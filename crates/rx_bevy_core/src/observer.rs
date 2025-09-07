use crate::SignalContext;

pub trait ObserverInput {
	type In: 'static;
	type InError: 'static;
}

impl ObserverInput for () {
	type In = ();
	type InError = ();
}

pub trait Observer: ObserverInput + SignalContext {
	fn next<'c>(&mut self, next: Self::In, context: &mut Self::Context<'c>);
	fn error<'c>(&mut self, error: Self::InError, context: &mut Self::Context<'c>);
	fn complete<'c>(&mut self, context: &mut Self::Context<'c>);

	/// Special fourth channel to process ticks issued by the schedulers.
	/// Some operators may produce other, new signals during a tick.
	/// None of the regular operators do anything on a tick but notify it's
	/// downstream of the tick.
	fn tick<'c>(&mut self, tick: crate::Tick, context: &mut Self::Context<'c>);
}
