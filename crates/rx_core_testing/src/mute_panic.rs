/// Replaces the panic hook with a noop for the duration of the function.
/// Useful for `#[should_panic]` tests, to ensure backtraces don't pollute
/// stdout.
pub fn mute_panic(fun: impl FnOnce()) {
	let hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(|_| {}));
	fun();
	std::panic::set_hook(hook);
}
