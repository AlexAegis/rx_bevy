#[derive(Clone, Default)]
pub struct KeyboardObservableOptions {
	pub emit: KeyboardObservableEmit,
}

#[derive(Default, Clone)]
pub enum KeyboardObservableEmit {
	#[default]
	JustPressed,
	WhilePressed,
	JustReleased,
}
