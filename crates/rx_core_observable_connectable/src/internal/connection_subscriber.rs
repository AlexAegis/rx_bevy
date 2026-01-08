use rx_core_common::{
	RxObserver, SubjectLike, SubscriptionData, SubscriptionLike, Teardown, TeardownCollection,
};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(Connector::In)]
#[rx_in_error(Connector::InError)]
pub struct ConnectionSubscriber<Connector>
where
	Connector: 'static + SubjectLike,
{
	connector: Connector,
	on_error: Option<Box<dyn FnOnce() + Send + Sync>>,
	on_complete: Option<Box<dyn FnOnce() + Send + Sync>>,
	on_unsubscribe: Option<Box<dyn FnOnce() + Send + Sync>>,
	teardown: SubscriptionData,
}

impl<Connector> ConnectionSubscriber<Connector>
where
	Connector: 'static + SubjectLike,
{
	pub(crate) fn new(
		connector: Connector,
		on_complete: Box<dyn FnOnce() + Send + Sync>,
		on_error: Box<dyn FnOnce() + Send + Sync>,
		on_unsubscribe: Box<dyn FnOnce() + Send + Sync>,
	) -> Self {
		Self {
			connector,
			on_complete: Some(on_complete),
			on_error: Some(on_error),
			on_unsubscribe: Some(on_unsubscribe),
			teardown: SubscriptionData::default(),
		}
	}
}

impl<Connector> RxObserver for ConnectionSubscriber<Connector>
where
	Connector: 'static + SubjectLike,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			self.connector.next(next);
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			if let Some(on_error) = self.on_error.take() {
				on_error()
			}
			self.connector.error(error);
			self.on_complete.take(); // Once errored, can't complete
		}
	}

	#[inline]
	fn complete(&mut self) {
		if !self.is_closed() {
			if let Some(on_complete) = self.on_complete.take() {
				on_complete()
			}
			self.connector.complete();
			self.on_error.take(); // Once completed, can't error
		}
	}
}

impl<Connector> TeardownCollection for ConnectionSubscriber<Connector>
where
	Connector: 'static + SubjectLike,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.teardown.add_teardown(teardown);
	}
}

impl<Connector> SubscriptionLike for ConnectionSubscriber<Connector>
where
	Connector: 'static + SubjectLike,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed() || self.connector.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.on_complete.take(); // In case wasn't completed
			self.on_error.take(); // In case didn't error
			if let Some(on_unsubscribe) = self.on_unsubscribe.take() {
				on_unsubscribe()
			}
			self.teardown.unsubscribe();
			// Disconnected from the connector!
			// A connection must not unsubscribe the connector!
			// Even though Subjects are expected to already upgrade to a detached
			// subscriber, other unknown subject implementations may ignore that.
		}
	}
}
