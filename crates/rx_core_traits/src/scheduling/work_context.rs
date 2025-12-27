use std::time::Duration;

pub trait WithWorkContextProvider {
	type WorkContextProvider: WorkContextProvider;
}

pub trait WorkContextProvider {
	type Item<'c>: WorkContext<'c>;
}

pub trait WorkContext<'c> {
	fn now(&self) -> Duration;
}

pub trait WithWorkInputOutput {
	/// Some schedulers pass inputs - such as the time passed - into the work
	/// to advance it.
	type Tick;
}
