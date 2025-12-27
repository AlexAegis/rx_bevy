use rx_core::prelude::*;

#[derive(PartialEq, Clone, Debug)]
enum ExampleProvenance {
	Foo,
	Bar,
}

fn main() {
	let mut subject =
		ProvenanceSubject::<ExampleProvenance, usize>::new(10, ExampleProvenance::Foo);

	let _all_subscription = subject
		.clone()
		.all()
		.subscribe(PrintObserver::<usize>::new("provenance_ignored"));

	let _bar_subscription = subject
		.clone()
		.only_by_provenance(ExampleProvenance::Bar)
		.subscribe(PrintObserver::<usize>::new("provenance_bar"));

	let _foo_subscription = subject
		.clone()
		.only_by_provenance(ExampleProvenance::Foo)
		.subscribe(PrintObserver::<usize>::new("provenance_foo"));

	subject.next((1, ExampleProvenance::Foo));
	subject.next((2, ExampleProvenance::Bar));
	subject.next((3, ExampleProvenance::Foo));
	subject.next((4, ExampleProvenance::Bar));
}
