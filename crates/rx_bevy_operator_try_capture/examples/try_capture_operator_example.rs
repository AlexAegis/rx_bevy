use rx_bevy::prelude::*;
use rx_bevy_operator_try_capture::prelude::ObservableExtensionTryCapture;

/// The [TryCaptureOperator] is used to pack incoming values and errors into a
/// Result. When used, upstream errors are guaranteed to not reach downstream.
fn main() {
	let _s = throw("non even!".to_string())
		.try_capture()
		.subscribe(PrintObserver::new("try_capture_operator"));
}
