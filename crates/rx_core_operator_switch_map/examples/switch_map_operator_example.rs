use rx_core::prelude::*;

fn main() {
	let _s = (1..=5)
		.into_observable::<()>()
		.switch_map(|next| IteratorObservable::new(next..=3))
		.subscribe(PrintObserver::new("switch_map"), &mut ());
}
