use rx_bevy::prelude::*;

/// The [LiftResultOperator] is used to unpack an incoming Result<T, E> into T
/// if it's Ok(T) and next it, and if it's an Err(E), downstream will receive it
/// as an error.
fn main() {
	let _s = (1..=5)
		.into_observable()
		.map(|i| {
			if i % 2 == 0 {
				Result::<i32, String>::Ok(i)
			} else {
				Result::<i32, String>::Err("Not Even!".to_string())
			}
		})
		.lift_result(|_in_error: ()| unreachable!())
		.subscribe(PrintObserver::new("lift_result_operator"));
}
