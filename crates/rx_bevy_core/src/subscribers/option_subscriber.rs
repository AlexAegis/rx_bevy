use crate::{Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown, WithContext};

pub enum OptionSubscriber<InnerSubscriber, Destination>
where
	InnerSubscriber: Subscriber,
	Destination: Subscriber<
			In = InnerSubscriber::In,
			InError = InnerSubscriber::InError,
			Context = InnerSubscriber::Context,
		>,
{
	Some(InnerSubscriber),
	None(Destination),
}

impl<InnerSubscriber, Destination> ObserverInput for OptionSubscriber<InnerSubscriber, Destination>
where
	InnerSubscriber: Subscriber,
	Destination: Subscriber<
			In = InnerSubscriber::In,
			InError = InnerSubscriber::InError,
			Context = InnerSubscriber::Context,
		>,
{
	type In = InnerSubscriber::In;
	type InError = InnerSubscriber::InError;
}

impl<InnerSubscriber, Destination> WithContext for OptionSubscriber<InnerSubscriber, Destination>
where
	InnerSubscriber: Subscriber,
	Destination: Subscriber<
			In = InnerSubscriber::In,
			InError = InnerSubscriber::InError,
			Context = InnerSubscriber::Context,
		>,
	InnerSubscriber::In: 'static,
	InnerSubscriber::InError: 'static,
{
	type Context = InnerSubscriber::Context;
}

impl<InnerSubscriber, Destination> Observer for OptionSubscriber<InnerSubscriber, Destination>
where
	InnerSubscriber: Subscriber,
	Destination: Subscriber<
			In = InnerSubscriber::In,
			InError = InnerSubscriber::InError,
			Context = InnerSubscriber::Context,
		>,
	InnerSubscriber::In: 'static,
	InnerSubscriber::InError: 'static,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => internal_subscriber.next(next, context),
			OptionSubscriber::None(fallback_subscriber) => fallback_subscriber.next(next, context),
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => {
				internal_subscriber.error(error, context)
			}
			OptionSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.error(error, context)
			}
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => internal_subscriber.complete(context),
			OptionSubscriber::None(fallback_subscriber) => fallback_subscriber.complete(context),
		}
	}

	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => internal_subscriber.tick(tick, context),
			OptionSubscriber::None(fallback_subscriber) => fallback_subscriber.tick(tick, context),
		}
	}
}

impl<InnerSubscriber, Destination> SubscriptionLike
	for OptionSubscriber<InnerSubscriber, Destination>
where
	InnerSubscriber: Subscriber,
	Destination: Subscriber<
			In = InnerSubscriber::In,
			InError = InnerSubscriber::InError,
			Context = InnerSubscriber::Context,
		>,
	InnerSubscriber::In: 'static,
	InnerSubscriber::InError: 'static,
{
	fn is_closed(&self) -> bool {
		match self {
			OptionSubscriber::Some(internal_subscriber) => internal_subscriber.is_closed(),
			OptionSubscriber::None(fallback_subscriber) => fallback_subscriber.is_closed(),
		}
	}

	fn unsubscribe(&mut self, context: &mut InnerSubscriber::Context) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => {
				internal_subscriber.unsubscribe(context);
			}
			OptionSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.unsubscribe(context);
			}
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => {
				internal_subscriber.add_teardown(teardown, context);
			}
			OptionSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.add_teardown(teardown, context);
			}
		}
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		match self {
			OptionSubscriber::Some(internal_subscriber) => {
				internal_subscriber.get_context_to_unsubscribe_on_drop()
			}
			OptionSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.get_context_to_unsubscribe_on_drop()
			}
		}
	}
}
