use rx_core::prelude::*;

fn main() {
	let _s = never().subscribe(PrintObserver::new("never"));
	println!("nothing happens before dropping the subscription!")
}
