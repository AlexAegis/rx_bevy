use rx_core::prelude::*;

fn main() {
	let _s = never().subscribe(PrintObserver::default());
	println!("nothing happens before dropping the subscription..")
}
