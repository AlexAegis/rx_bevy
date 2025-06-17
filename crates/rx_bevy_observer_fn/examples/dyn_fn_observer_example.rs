use rx_bevy::prelude::*;

/// An [DynFnObserver] can have its notifiers configured dynamically
fn main() {
	of("world").subscribe(DynFnObserver::default().with_next(|next| println!("hello {next}")));
}
