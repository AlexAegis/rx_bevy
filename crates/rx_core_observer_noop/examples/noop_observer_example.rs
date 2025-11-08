use rx_core::prelude::*;

/// The [NoopObserver] does nothing with the received values
fn main() {
	let mut context = ();
	let _s = of::<_, ()>(1).subscribe(NoopObserver::default(), &mut context);
}
