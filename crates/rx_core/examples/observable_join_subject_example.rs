use rx_core::prelude::*;

fn main() {
	let mut greetings_subject = PublishSubject::<&'static str>::default();
	let mut count_subject = PublishSubject::<usize>::default();

	let mut subscription = join(greetings_subject.clone(), count_subject.clone())
		.subscribe(PrintObserver::new("join"));

	greetings_subject.next("Hello!");
	count_subject.next(10);
	count_subject.next(20);
	greetings_subject.next("Szia!");
	count_subject.next(30);
	greetings_subject.complete();
	count_subject.complete();
	subscription.unsubscribe();
}
