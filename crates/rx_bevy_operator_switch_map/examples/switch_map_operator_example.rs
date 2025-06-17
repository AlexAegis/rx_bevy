use rx_bevy::prelude::*;
use rx_bevy_operator_switch_map::switch_map_extension::ObservableExtensionSwitchMap;

fn main() {
	// TODO: Fix this
	(1..=5)
		.into_observable()
		.switch_map(|next| {
			IteratorObservable::new(next..=3).map(move |i| format!("from {next} i: {i}"))
		})
		.subscribe(PrintObserver::new("switch_map"))
}
