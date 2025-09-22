use rx_bevy::{ArcSubscriber, ErasedArcSubscriber, prelude::*};

fn main() {
	let _s = (1..=5)
		.into_observable()
		.switch_map(
			|next| IteratorObservable::new(next..=3),
			use_share::<ErasedArcSubscriber<i32, (), ()>>(),
		)
		.subscribe(PrintObserver::new("switch_map"), &mut ());
}
