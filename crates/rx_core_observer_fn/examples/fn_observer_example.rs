use rx_core::prelude::*;

/// An [FnObserver] requires you to define a callback for all three notifications
fn main() {
	let _s = of("world").subscribe(FnObserver::new(
		|next| println!("hello: {next}"),
		|_error| println!("error"),
		|| {},
	));
}
