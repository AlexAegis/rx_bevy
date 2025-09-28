use rx_bevy::{ErasedArcSubscriber, prelude::*};
use rx_bevy_operator_switch_map::switch_map_extension_pipe::ObservableExtensionSwitchMap;

fn main() {
	let _s = (1..=3)
		.into_observable()
		.finalize(|_context| println!("first finalize")) // TODO: Fix, this finalizer isn't running
		.tap_next(|n, _context| println!("1n {n}"))
		.switch_map(
			|next| {
				IteratorObservable::new(next..=3)
					.map(move |i| format!("from {next} i: {i}"))
					.tap_next(|n, _context| println!("2n {n}"))
					.finalize(|_context| println!("inner finalize")) // TODO: Fix, this finalizer isn't running
			},
			use_sharer::<ErasedArcSubscriber<_, _, _>>(),
		)
		.finalize(|_context| println!("last finalize")) // TODO: Fix, this finalizer isn't running
		.tap_next(|n, _context| println!("3n {n}"))
		.subscribe(PrintObserver::new("switch_map"), &mut ());
}
