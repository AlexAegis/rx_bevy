use rx_core::prelude::*;

/// The [CombineChangesObserver] combines the latest values from multiple other
/// observables.
fn main() {
	let mut greetings_subject = PublishSubject::<&'static str>::default();
	let mut count_subject = PublishSubject::<usize>::default();

	let mut subscription = combine_changes(greetings_subject.clone(), count_subject.clone())
		.subscribe(PrintObserver::new("combine_changes"));

	greetings_subject.next("Hello!");
	count_subject.next(10);
	count_subject.next(20);
	greetings_subject.next("Szia!");
	greetings_subject.complete();
	count_subject.next(30);
	count_subject.complete();
	subscription.unsubscribe();
}
