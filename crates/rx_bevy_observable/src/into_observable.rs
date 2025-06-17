use crate::Observable;

pub trait IntoObservable {
	type Obs: Observable;

	fn into_observable(self) -> Self::Obs;
}
