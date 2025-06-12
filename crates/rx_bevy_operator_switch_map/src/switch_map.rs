use std::marker::PhantomData;

pub struct SwitchMapOperator<In, InError, InnerObservable, Switcher> {
	pub switcher: Switcher,
	pub _phantom_data: PhantomData<(In, InError, InnerObservable)>,
}

impl<In, InError, OutObservable, Switcher> SwitchMapOperator<In, InError, OutObservable, Switcher> {
	pub fn new(switcher: Switcher) -> Self {
		Self {
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OutObservable, Switcher> Clone
	for SwitchMapOperator<In, InError, OutObservable, Switcher>
where
	Switcher: Clone,
{
	fn clone(&self) -> Self {
		Self {
			switcher: self.switcher.clone(),
			_phantom_data: PhantomData,
		}
	}
}

pub struct SwitchMapSubscriber<In, InError, InnerObservable, Switcher>
where
	Switcher: Clone + Fn(In) -> InnerObservable,
{
	switcher: Switcher,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, InnerObservable, Switcher>
	SwitchMapSubscriber<In, InError, InnerObservable, Switcher>
where
	Switcher: Clone + Fn(In) -> InnerObservable,
{
	pub fn new(switcher: Switcher) -> Self {
		Self {
			switcher,
			_phantom_data: PhantomData,
		}
	}
}
