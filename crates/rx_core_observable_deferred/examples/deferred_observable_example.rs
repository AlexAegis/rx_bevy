use std::cell::RefCell;

use rx_core::prelude::*;

/// The deferred observable creates the observable you subscribe to, when you
/// subscribe to it.
fn main() {
	let i = RefCell::new(1);
	let mut deferred = deferred_observable(|| {
		println!("subscribe!");
		(0..=*i.borrow()).into_observable::<()>()
	});
	// The thing the inner observable depends on is allowed to change between declaration and subscription!
	*i.borrow_mut() = 2;
	let _s = deferred.subscribe(PrintObserver::new("deferred_observable"), &mut ());
}
