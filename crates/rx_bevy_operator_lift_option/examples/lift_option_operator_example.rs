use rx_bevy::prelude::*;
use rx_bevy_operator_lift_option::prelude::ObservableExtensionLiftOption;

/// The [LiftOptionOperator] is used to unpack an incoming Option<T> into T if it's Some(T)
/// When the incoming option is None, the downstream won't be notified.
/// In this example you can see what the `filter_map` operator does under the hood.
fn main() {
	let _s = (1..=5)
		.into_observable()
		.map(|i| if i % 2 == 0 { Some(i) } else { None })
		.lift_option()
		.subscribe(PrintObserver::new("lift_option_operator"), &mut ());
}
