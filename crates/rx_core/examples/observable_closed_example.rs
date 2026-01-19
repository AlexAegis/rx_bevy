use rx_core::prelude::*;

fn main() {
	let _s = closed().subscribe(PrintObserver::new("closed"));
	println!("end")
}
