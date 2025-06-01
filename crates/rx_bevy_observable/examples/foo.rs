use rx_bevy_observable::{DynObserver, Observable, Observer, Subscriber};
use rx_bevy_observer_fn::FnObserver;

pub struct FnObservable<T, F>
where
	F: FnMut() -> T,
{
	pub factory: F,
}

impl<T, F> Observable for FnObservable<T, F>
where
	F: Clone + Fn() -> T,
{
	type Out = T;

	type Subscription = Subscriber<DynObserver<T>>;

	fn subscribe<Destination: 'static + Observer<Self::Out>>(
		&mut self,
		mut observer: Destination,
	) -> Self::Subscription {
		let mut sub = Subscriber {
			destination: Some(DynObserver {
				dyn_on_push: Box::new(move |next| {
					observer.on_push(next);
				}),
			}),
		};

		sub.on_push((self.factory)());
		sub
	}
}

fn main() {
	let mut fn_observable = FnObservable {
		factory: Box::new(|| 12),
	};

	let hello_observer = FnObserver::new(|next: i32| println!("hello {next}"));

	fn_observable.subscribe(hello_observer);
}
