// pub trait Operator<In, Out>: Observer<In> + Observable<Observer<In>, Out> {}

// pub(crate) struct OperatorConfig<In, Out> {
// 	pub(crate) source: ObservableContainer<In>,
// 	pub(crate) destination: ObserverContainer<Out>,
// }
/*
impl<In, Out> Observable<Out> for OperatorConfig<In, Out> {
	fn subscribe_container(&mut self, observer: ObserverContainer<Out>) {
		self.destination = observer;
	}
}

impl<In, Out> Observer<Out> for OperatorConfig<In, Out> {
	fn on_push(&mut self, value: Out) {
		let mut lock = self.source.observable.write().expect("BASDASD");
		self.source.subscribe_container();
	}
}
*/
