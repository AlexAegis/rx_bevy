use rx_bevy::{ErasedArcSubscriber, prelude::*};

fn main() {
	let _s = (1..=5)
		.into_observable()
		.switch_map(
			|next| IteratorObservable::new(next..=3),
			use_sharer::<ErasedArcSubscriber<_, _, _>>(),
		)
		.subscribe(PrintObserver::new("switch_map"), &mut ());
}
