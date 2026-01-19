use rx_core::prelude::*;
use rx_core_common::Never;

fn main() {
	let _s = create_observable::<&str, Never, _>(|destination| {
		destination.next("hello");
		destination.complete();
	})
	.subscribe(PrintObserver::new("create_observable"));
}
