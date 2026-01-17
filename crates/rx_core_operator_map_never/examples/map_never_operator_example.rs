use rx_core::prelude::*;

fn main() {
	let _s = throw("error")
		.map_never()
		.subscribe(PrintObserver::<i32, &'static str>::new("map_never (next)"));

	let _s = just(1)
		.map_never()
		.subscribe(PrintObserver::<i32, &'static str>::new("map_never (error)"));

	let _s = empty()
		.map_never_both()
		.subscribe(PrintObserver::<i32, &'static str>::new("map_never_both"));
}
