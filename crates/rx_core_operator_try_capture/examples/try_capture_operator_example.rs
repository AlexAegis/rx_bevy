use rx_core::prelude::*;

/// The [TryCaptureOperator] is used to pack incoming values and errors into a
/// Result. When used, upstream errors are guaranteed not to reach downstream.
fn main() {
	let _s = throw::<_, ()>("error!".to_string())
		.try_capture()
		.subscribe(PrintObserver::new("try_capture_operator"), &mut ());
}
