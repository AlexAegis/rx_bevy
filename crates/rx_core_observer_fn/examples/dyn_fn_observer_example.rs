use rx_core::prelude::*;

/// An [DynFnObserver] can have its notifiers configured dynamically
fn main() {
	let _s = just("world")
		.subscribe(DynFnObserver::default().with_next(|next| println!("hello {next}")));
}
