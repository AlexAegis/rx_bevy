use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_notification_store::NotificationState;
use rx_core_notification_variadics::EitherObservableNotification2;
use rx_core_traits::{Observable, Observer, Subscriber, SubscriberNotification, SubscriptionLike};

#[derive(RxSubscriber)]
#[rx_in(EitherObservableNotification2<O1, O2>)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection_to_destination]
pub struct CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	o1_state: NotificationState<O1::Out, O1::OutError>,
	o2_state: NotificationState<O2::Out, O2::OutError>,
	#[destination]
	destination: Destination,
}

impl<Destination, O1, O2> CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	pub fn new(destination: Destination) -> Self {
		CombineLatestSubscriber {
			o1_state: NotificationState::default(),
			o2_state: NotificationState::default(),
			destination,
		}
	}

	fn try_complete(&mut self) {
		if (self.o1_state.is_completed() && self.o2_state.is_completed())
			|| (self.o1_state.is_waiting() && self.o2_state.is_completed_but_not_primed())
			|| (self.o1_state.is_completed_but_not_primed() && self.o2_state.is_waiting())
		{
			self.destination.complete();
			self.destination.unsubscribe();
		}
	}

	fn try_unsubscribe(&mut self) {
		if (self.o1_state.is_closed() && self.o2_state.is_closed())
			|| (self.o1_state.is_waiting()
				&& self.o2_state.is_closed_but_not_primed_and_not_completed())
			|| (self.o1_state.is_closed_but_not_primed_and_not_completed()
				&& self.o2_state.is_waiting())
		{
			self.destination.unsubscribe();
		}
	}
}

impl<Destination, O1, O2> Observer for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	fn next(&mut self, next: Self::In) {
		let either_was_next = match next {
			EitherObservableNotification2::O1(notification) => {
				let is_next = matches!(notification, SubscriberNotification::Next(_));
				self.o1_state.push(notification);
				is_next
			}
			EitherObservableNotification2::O2(notification) => {
				let is_next = matches!(notification, SubscriberNotification::Next(_));
				self.o2_state.push(notification);
				is_next
			}
		};

		if let Some(error) = self
			.o1_state
			.take_error()
			.map(|error| error.into())
			.or_else(|| self.o2_state.take_error().map(|error| error.into()))
		{
			self.destination.error(error);
			self.destination.unsubscribe();
			return;
		}

		self.try_complete();
		self.try_unsubscribe();

		if either_was_next
			&& !self.is_closed()
			&& let Some((o1_val, o2_val)) = self.o1_state.get_value().zip(self.o2_state.get_value())
		{
			self.destination.next((o1_val.clone(), o2_val.clone()));
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.destination.error(error);
			self.unsubscribe()
		}
	}

	fn complete(&mut self) {
		self.try_complete();
	}
}

impl<Destination, O1, O2> SubscriptionLike for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.try_unsubscribe();
	}
}
