pub trait Observer {
	type In;

	fn on_push(&mut self, value: Self::In);
}
