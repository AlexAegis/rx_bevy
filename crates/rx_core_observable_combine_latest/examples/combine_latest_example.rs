use rx_core::prelude::*;

/// The [CombineLatestObserver] combines the latest values from multiple other
/// observables.
fn main() {
	let mut greetings_subject = PublishSubject::<&'static str>::default();
	let mut count_subject = PublishSubject::<usize>::default();

	let mut subscription = combine_latest(
		greetings_subject
			.clone()
			.tap(PrintObserver::new("greetings_subject")),
		count_subject
			.clone()
			.tap(PrintObserver::new("count_subject")),
	)
	.subscribe(PrintObserver::new("combine_latest"));

	greetings_subject.next("Hello!");
	count_subject.next(10);
	count_subject.next(20);
	greetings_subject.next("Szia!");
	greetings_subject.complete();
	count_subject.next(30);
	count_subject.complete();
	subscription.unsubscribe();
}
