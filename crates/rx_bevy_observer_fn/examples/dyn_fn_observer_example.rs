use rx_bevy::prelude::*;

/// An [DynFnObserver] can have its notifiers configured dynamically
fn main() {
	let _s =
		of("world").subscribe(DynFnObserver::default().with_next(|next| println!("hello {next}")));
}
