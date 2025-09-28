use rx_bevy::{ErasedArcSubscriber, prelude::*};
use rx_bevy_operator_switch_map::switch_map_extension_pipe::ObservableExtensionSwitchMap;

fn main() {
	let _s = (1..=3)
		.into_observable()
		.finalize(|_| println!("fir fin"))
		.tap_next(|n| println!("1n {n}"))
		.switch_map(
			|next| {
				IteratorObservable::new(next..=3)
					.map(move |i| format!("from {next} i: {i}"))
					.tap_next(|n| println!("2n {n}"))
					.finalize(|_| println!("inner fin"))
			},
			use_sharer::<ErasedArcSubscriber<_, _, _>>(),
		)
		.finalize(|_| println!("lat fin"))
		.tap_next(|n| println!("3n {n}"))
		.take(4)
		.subscribe(PrintObserver::new("switch_map"), &mut ());
}
