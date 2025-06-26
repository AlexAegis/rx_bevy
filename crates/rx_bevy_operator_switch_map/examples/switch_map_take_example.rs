use rx_bevy::prelude::*;
use rx_bevy_operator_switch_map::switch_map_extension_pipe::ObservableExtensionSwitchMap;

fn main() {
	let _s = (1..=5)
		.into_observable()
		.finalize(|| println!("fir fin"))
		.tap_next(|n| println!("1n {n}"))
		.switch_map(|next| {
			IteratorObservable::new(next..=3)
				.map(move |i| format!("from {next} i: {i}"))
				.tap_next(|n| println!("2n {n}"))
				.finalize(|| println!("inner fin"))
		})
		.finalize(|| println!("lat fin"))
		.tap_next(|n| println!("3n {n}"))
		.take(4)
		.subscribe(PrintObserver::new("switch_map"));
}
