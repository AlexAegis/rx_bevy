use rx_core::prelude::*;

/// An [DynFnObserver] can have its notifiers configured dynamically
fn main() {
	let _s = of("world").subscribe(
		DynFnObserver::default().with_next(|next, _context| println!("hello {next}")),
		&mut (),
	);
}
