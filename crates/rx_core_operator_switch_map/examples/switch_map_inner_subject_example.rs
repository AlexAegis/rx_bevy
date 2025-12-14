use rx_core::prelude::*;

#[derive(Clone, Debug)]
enum Either {
	Left,
	Right,
}

fn main() {
	let mut upstream_subject = PublishSubject::<Either>::default();
	let mut inner_left_subject = PublishSubject::<i32>::default();
	let mut inner_right_subject = PublishSubject::<i32>::default();

	let l = inner_left_subject.clone();
	let r = inner_right_subject.clone();
	let mut subscription = upstream_subject
		.clone()
		.finalize(|| println!("finalize: upstream"))
		.tap_next(|n| println!("emit (source): {n:?}"))
		.switch_map(move |next| match next {
			Either::Left => l.clone(),
			Either::Right => r.clone(),
		})
		.finalize(|| println!("finalize: downstream"))
		.subscribe(PrintObserver::new("switch_map"));

	upstream_subject.next(Either::Left);
	inner_left_subject.next(1);
	inner_right_subject.next(2);
	inner_left_subject.next(3);
	inner_right_subject.next(4);
	inner_left_subject.complete();
	upstream_subject.next(Either::Right);
	inner_left_subject.next(5);
	inner_right_subject.next(6);
	upstream_subject.complete();
	inner_left_subject.next(7);
	inner_right_subject.next(8);
	inner_right_subject.complete();

	upstream_subject.unsubscribe();
	subscription.unsubscribe();
}
