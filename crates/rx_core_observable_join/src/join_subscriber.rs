use rx_core_common::{Observable, Observer, Subscriber, SubscriptionLike};
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_notification_store::NotificationState;
use rx_core_notification_variadics::EitherObservableNotification2;

const UNREACHABLE_ERROR: &str = "The JoinSubscriber expects only materialized notifications through its `next` fn, from an EitherSubscriber.";

/// # JoinSubscriber
///
/// From an upstream multiplexer over two source observables, this
/// subscriber maintains a state for each sources last emission separately,
/// and emits a tuple of them once both of them complete and both of them have
/// emitted a value before completing.
///
/// Meaning if even one of the observables haven't emitted before all of them
/// had completed, only a complete notification will be observed!
///
/// If not all observables complete, nothing will be emitted even if all
/// input observables were primed.
#[derive(RxSubscriber)]
#[rx_in(EitherObservableNotification2<O1, O2>)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl] // This subscribers unsubscribe method should be unreachable!
pub struct JoinSubscriber<Destination, O1, O2>
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

impl<Destination, O1, O2> JoinSubscriber<Destination, O1, O2>
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
		JoinSubscriber {
			o1_state: NotificationState::default(),
			o2_state: NotificationState::default(),
			destination,
		}
	}

	fn get_next(&self) -> Option<Destination::In> {
		self.o1_state
			.get_value()
			.zip(self.o2_state.get_value())
			.map(|(o1, o2)| (o1.clone(), o2.clone()))
	}

	fn take_either_error(&mut self) -> Option<Destination::InError> {
		self.o1_state
			.take_error()
			.map(|error| error.into())
			.or_else(|| self.o2_state.take_error().map(|error| error.into()))
	}

	fn try_complete(&mut self) {
		if !self.destination.is_closed()
			&& (self.o1_state.is_completed() && self.o2_state.is_completed())
		{
			if let Some(next) = self.get_next() {
				self.destination.next(next);
			}

			self.destination.complete();
		}
	}

	fn try_unsubscribe(&mut self) {
		if !self.destination.is_closed() && (self.o1_state.is_closed() && self.o2_state.is_closed())
		{
			self.destination.unsubscribe();
		}
	}
}

impl<Destination, O1, O2> Observer for JoinSubscriber<Destination, O1, O2>
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
		match next {
			EitherObservableNotification2::O1(notification) => {
				if !self
					.o1_state
					.update_with_notification_would_be_invalid(&notification)
				{
					self.o1_state.push(notification);
				}
			}
			EitherObservableNotification2::O2(notification) => {
				if !self
					.o2_state
					.update_with_notification_would_be_invalid(&notification)
				{
					self.o2_state.push(notification);
				}
			}
		};

		if let Some(error) = self.take_either_error() {
			self.destination.error(error);
			return;
		}

		self.try_complete();
		self.try_unsubscribe();
	}

	fn error(&mut self, _error: Self::InError) {
		unreachable!("{} - Error", UNREACHABLE_ERROR)
	}

	fn complete(&mut self) {
		unreachable!("{} - Complete", UNREACHABLE_ERROR)
	}
}

impl<Destination, O1, O2> SubscriptionLike for JoinSubscriber<Destination, O1, O2>
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

	fn unsubscribe(&mut self) {
		unreachable!("{} - Unsubscribe", UNREACHABLE_ERROR)
	}
}
