use rx_core::prelude::*;

/// Generic operators can be passed into the pipe function
fn main() {
	let observable = OfObservable::<i32, ()>::new(12);
	let mut pipe = observable
		.pipe(MapOperator::new(|n: i32| -> i32 { n * 2 }))
		.pipe(MapOperator::new(|n: i32| -> String { n.to_string() }));

	let observer = DynFnObserver::default().with_next(|next, _context| println!("{next}"));

	let _s = pipe.subscribe(observer, &mut ());
}
