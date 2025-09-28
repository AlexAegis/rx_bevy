use rx_bevy::{DropContext, DropUnsafeSignalContext, ErasedArcSubscriber, prelude::*};
use rx_bevy_operator_switch_map::switch_map_extension_pipe::ObservableExtensionSwitchMap;

struct CustomContext;

impl DropContext for CustomContext {
	type DropSafety = DropUnsafeSignalContext;

	fn get_context_for_drop() -> Self {
		panic!("Don't worry about me");
	}
}

/// Since all subscriptions present here are inert, it's safe to use an drop-unsafe context
fn main() {
	let mut context = CustomContext;

	let mut subscription = (1..=3)
		.into_observable::<CustomContext>()
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
		.finalize(|_context| println!("last finalize"))
		.tap_next(|n, _context| println!("3n {n}"))
		.subscribe(PrintObserver::new("switch_map"), &mut context);
	subscription.unsubscribe(&mut context);
}
