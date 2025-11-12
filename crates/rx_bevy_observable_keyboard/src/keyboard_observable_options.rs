#[derive(Default, Clone)]
pub struct KeyboardObservableOptions {
	pub emit: KeyboardObservableEmit,
}

#[derive(Default, Clone)]
pub enum KeyboardObservableEmit {
	#[default]
	JustPressed,
	Pressed,
	JustReleased,
}
