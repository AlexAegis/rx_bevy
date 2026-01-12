use rx_core::prelude::*;

fn main() {
	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut inner = PublishSubject::<&'static str, &'static str>::default();

	let _s = source
		.clone()
		.with_latest_from(inner.clone())
		.subscribe(PrintObserver::new("with_latest_from_operator"));

	source.next(1);

	inner.next("hello");

	source.next(2);
	source.next(3);
	source.next(4);

	inner.next("bello");

	source.next(5);
	inner.error("error");
}
