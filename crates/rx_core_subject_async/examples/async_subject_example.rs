use rx_core::prelude::*;

fn main() {
	let mut subject = AsyncSubject::<i32>::default();

	let mut _subscription_1 = subject
		.clone()
		.subscribe(PrintObserver::<i32>::new("async_subject sub_1"));

	subject.next(1);
	subject.next(2);

	let mut _subscription_2 = subject
		.clone()
		.subscribe(PrintObserver::<i32>::new("async_subject sub_2"));

	subject.next(3);
	subject.complete();
}
