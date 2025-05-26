use std::marker::PhantomData;

use rx_bevy_operator::Operator;

#[derive(Debug)]
pub struct TapOperator<In, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	callback: Callback,
	_phantom_data: PhantomData<In>,
}

impl<In, Callback> Operator for TapOperator<In, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type In = In;
	type Out = In;

	type Instance = Self;

	fn create_operator_instance(&self) -> Self::Instance {
		self.clone()
	}

	fn operate(&mut self, next: Self::In) -> Self::Out {
		(self.callback)(&next);
		next
	}
}

impl<In, Callback> TapOperator<In, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	pub fn new(callback: Callback) -> Self {
		Self {
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Callback> Clone for TapOperator<In, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data: PhantomData,
		}
	}
}
