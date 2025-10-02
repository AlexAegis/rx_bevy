use rx_bevy::{ErasedArcSubscriber, prelude::*};
use rx_bevy_operator_switch_map::switch_map_extension_pipe::ObservableExtensionSwitchMap;

/// Since all subscriptions present here are inert, it's safe to use an drop-unsafe context
fn main() {
	let mut context = ();

	let mut subscription = (1..=3)
		.into_observable()
		//.finalize(|_context| println!("finalize: upstream"))
		.tap_next(|n, _context| println!("emit (source): {n}"))
		.switch_map(
			|next| {
				IteratorObservable::new(next..=3)
					.map(move |i| format!("from {next} through 3, current: {i}"))
					.finalize(|_context| println!("finalize: inner"))
					.tap_next(|n, _context| println!("emit (inner): '{n}'"))
			},
			use_sharer::<ErasedArcSubscriber<_, _, _>>(),
		)
		//.finalize(|_context| println!("finalize: downstream"))
		.subscribe(PrintObserver::new("switch_map"), &mut context);
	subscription.unsubscribe(&mut context);
}
