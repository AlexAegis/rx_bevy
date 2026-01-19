use rx_core::prelude::*;

fn main() {
	let _s = (1..=5)
		.into_observable()
		.map(|i| {
			if i <= 3 {
				Result::<i32, String>::Ok(i)
			} else {
				Result::<i32, String>::Err("Larger than 3!".to_string())
			}
		})
		.lift_result() // We're lifting the result error from the "next" channel, but we still have to deal with the upstream errors if they exist, this `unreachable!` is just here to ignore them.
		.subscribe(PrintObserver::new("lift_result_operator"));
}
