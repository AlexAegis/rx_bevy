pub trait Observer {
	type In;

	fn on_push(&mut self, next: Self::In);
}
