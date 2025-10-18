use rx_bevy::prelude::*;
use rx_core_operator_switch_map::switch_map_extension_pipe::ObservableExtensionSwitchMap;

fn main() {
	let mut context = ();

	let mut subscription = (1..=3)
		.into_observable::<()>()
		.finalize(|_context| println!("finalize: upstream"))
		.tap_next(|n, _context| println!("emit (source): {n}"))
		.switch_map(|next| {
			IteratorObservable::new(next..=3)
				.map(move |i| format!("from {next} through 3, current: {i}"))
				.finalize(|_context| println!("finalize: inner"))
				.tap_next(|n, _context| println!("emit (inner): '{n}'"))
		})
		.finalize(|_context| println!("finalize: downstream"))
		.subscribe(PrintObserver::new("switch_map"), &mut context);
	subscription.unsubscribe(&mut context);
}
