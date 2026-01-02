use std::time::Duration;

pub trait WithWorkContextProvider {
	type WorkContextProvider: WorkContextProvider;
}

pub trait WorkContextProvider {
	type Item<'c>: WorkContext<'c>;
}

pub trait WorkContext<'c> {}

pub trait WithWorkInputOutput {
	/// Some schedulers pass inputs - such as the time passed - into the work
	/// to advance it.
	type Tick: WorkTick;
}

pub trait WorkTick {
	/// Returns the current time as time elapsed since startup.
	fn now(&self) -> Duration;
}
