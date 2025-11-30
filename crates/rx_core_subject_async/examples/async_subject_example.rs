use rx_core::prelude::*;

fn main() {
	let mut subject = AsyncSubject::<i32>::default();
	let mut context = ();

	let mut _subscription_1 = subject.clone().subscribe(
		PrintObserver::<i32>::new("async_subject sub_1"),
		&mut context,
	);

	subject.next(1, &mut context);
	subject.next(2, &mut context);

	let mut _subscription_2 = subject.clone().subscribe(
		PrintObserver::<i32>::new("async_subject sub_2"),
		&mut context,
	);

	subject.next(3, &mut context);
	subject.complete(&mut context);
}
