pub trait Provider {
	type Provided;

	fn provide(&self) -> Self::Provided;
}

pub trait ProviderMut {
	type Provided;

	fn provide(&mut self) -> Self::Provided;
}
