use rx_core::prelude::*;

/// An [FnObserver] requires you to define a callback for all three notifications
fn main() {
	let _s = of("world").subscribe(
		FnObserver::new(
			|next, _context| println!("hello: {next}"),
			|_error, _context| println!("error"),
			|_context| {},
			|_tick, _context| {},
		),
		&mut (),
	);
}
