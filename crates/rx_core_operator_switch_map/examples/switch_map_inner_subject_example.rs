use rx_core::prelude::*;

#[derive(Clone, Debug)]
enum Either {
	Left,
	Right,
}

fn main() {
	let mut context = ();

	let mut upstream_subject = Subject::<Either, ()>::default();
	let mut inner_left_subject = Subject::<i32, ()>::default();
	let mut inner_right_subject = Subject::<i32, ()>::default();

	let l = inner_left_subject.clone();
	let r = inner_right_subject.clone();
	let mut subscription = upstream_subject
		.clone()
		.finalize(|_context| println!("finalize: upstream"))
		.tap_next(|n, _context| println!("emit (source): {n:?}"))
		.switch_map(move |next| match next {
			Either::Left => l.clone(),
			Either::Right => r.clone(),
		})
		.finalize(|_context| println!("finalize: downstream"))
		.subscribe(PrintObserver::new("switch_map"), &mut context);

	upstream_subject.next(Either::Left, &mut context);
	inner_left_subject.next(1, &mut context);
	inner_right_subject.next(2, &mut context);
	inner_left_subject.next(3, &mut context);
	inner_right_subject.next(4, &mut context);
	inner_left_subject.complete(&mut context);
	upstream_subject.next(Either::Right, &mut context);
	inner_left_subject.next(5, &mut context);
	inner_right_subject.next(6, &mut context);
	upstream_subject.complete(&mut context);
	inner_left_subject.next(7, &mut context);
	inner_right_subject.next(8, &mut context);
	inner_right_subject.complete(&mut context);

	upstream_subject.unsubscribe(&mut context);
	subscription.unsubscribe(&mut context);
}
