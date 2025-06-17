use rx_bevy::prelude::*;

/// An [FnObserver] requires you to define a callback for all three notifications
fn main() {
	of("world").subscribe(FnObserver::new(
		|next| println!("hello: {next}"),
		|_error| println!("error"),
		|| {},
	));
}
