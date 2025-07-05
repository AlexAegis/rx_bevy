use std::marker::PhantomData;

use bevy::prelude::*;
use rx_bevy::{ObserverInput, SubscriptionLike};
use smallvec::SmallVec;

use crate::{RxComplete, RxError, RxNext};

#[derive(Debug, Reflect)]
pub enum ObservedEvent<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	Next(In),
	Error(InError),
	Complete,
}

#[derive(Debug, Reflect)]
#[deprecated]
pub struct RxBufferedSubscriber<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	closed: bool,
	buffer: SmallVec<[ObservedEvent<In, InError>; 2]>,
	destination: Entity,
	#[reflect(ignore)]
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> RxBufferedSubscriber<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	pub fn new(destination: Entity) -> Self {
		Self {
			destination,
			buffer: SmallVec::new(),
			closed: false,
			_phantom_data: PhantomData,
		}
	}

	/// Flush observed events
	pub fn flush(&mut self, commands: &mut Commands) -> bool {
		let mut flushed_something = false;
		for observed_event in self.buffer.drain(..) {
			match observed_event {
				ObservedEvent::Next(next) => {
					commands.trigger_targets(RxNext(next), self.destination);
				}
				ObservedEvent::Error(error) => {
					commands.trigger_targets(RxError(error), self.destination);
				}
				ObservedEvent::Complete => {
					commands.trigger_targets(RxComplete, self.destination);
				}
			}
			flushed_something = true;
		}
		flushed_something
	}
}

impl<In, InError> ObserverInput for RxBufferedSubscriber<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> rx_bevy::Observer for RxBufferedSubscriber<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	fn next(&mut self, next: Self::In) {
		self.buffer.push(ObservedEvent::Next(next));
	}

	fn error(&mut self, error: Self::InError) {
		self.buffer.push(ObservedEvent::Error(error));
		self.unsubscribe();
	}

	fn complete(&mut self) {
		self.buffer.push(ObservedEvent::Complete);
		self.unsubscribe();
	}
}

impl<In, InError> rx_bevy::SubscriptionLike for RxBufferedSubscriber<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		self.closed = true;
	}

	fn add(&mut self, _subscription: &'static mut dyn rx_bevy::SubscriptionLike) {}
}
