use std::sync::{Arc, RwLock};

use rx_bevy_core::{SignalContext, SubscriptionLike};

use crate::MulticastDestination;

pub struct MulticastSubscription<Context> {
	teardown: Option<Box<dyn FnOnce(&mut Context)>>,
}

impl<Context> MulticastSubscription<Context> {
	pub fn new<In: 'static, InError: 'static>(
		key: usize,
		multicast_ref: Arc<RwLock<MulticastDestination<In, InError, Context>>>,
	) -> Self {
		let teardown = move |context| {
			let subscriber = {
				let mut write_multicast = multicast_ref.write().expect("blocked 1");
				write_multicast.take(key)
			};
			if let Some(mut subscriber) = subscriber {
				subscriber.unsubscribe(context);
			}
		};

		Self {
			teardown: Some(Box::new(teardown)),
		}
	}
}

impl<Context> SignalContext for MulticastSubscription<Context> {
	type Context = Context;
}

impl<Context> SubscriptionLike for MulticastSubscription<Context> {
	fn is_closed(&self) -> bool {
		self.teardown.is_none()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if let Some(teardown_fn) = self.teardown.take() {
			teardown_fn(context);
		};
	}
}
