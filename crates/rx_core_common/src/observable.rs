use crate::{
	Signal, Subscriber, SubscriptionWithTeardown, UpgradeableObserver, WithPrimaryCategory,
};

/// # [ObservableOutput]
///
/// Defines the outputs of an [Observable]. Also used for
/// [Operator][crate::Operator]s to define the new outputs once the operator is
/// applied.
pub trait ObservableOutput {
	type Out: Signal;
	type OutError: Signal;
}

/// # [Observable]
///
/// An observable is a signal-source-descriptor that can be subscribed to,
/// allowing you to observe its signals.
///
/// > Simply creating an observable instance will do nothing as it just defines
/// > how the subscriptions behave that it can create on subscribe.
///
/// ## Signals
///
/// Anything an observable can push is a signal, not just the values you
/// subscribe for, but errors, completions and unsubscribes too. These are the
/// actions an observable can take.
///
/// ## [Subscribe][Observable::subscribe]
///
/// To subscribe to an observable, you must provide a destination, an observer,
/// to which values and other observable signals will be forwarded to.
///
/// Calling [`subscribe`][Observable::subscribe] will result in a
/// [Subscription][crate::SubscriptionLike] that will contain the one (or more)
/// [Teardown][crate::Teardown]s that can be used to release resources
/// associated with this subscription.
///
/// ## Dropping Subscriptions
///
/// Subscriptions that were not unsubscribed when they are dropped will try to
/// unsubscribe themselves.
///
/// > Note that not assigning the subscription to a variable (or assigning it to
/// > `let _ =`) will cause it to be immediately dropped, hence `subscribe` is
/// > `#[must_use]`!
pub trait Observable: ObservableOutput + WithPrimaryCategory {
	/// The subscription produced by this [Observable]. As this is the only kind
	/// of subscription that is handled directly by users, only here are
	/// subscriptions required to implement [Drop] to ensure resources
	/// are released when the subscription is dropped.
	type Subscription<Destination>: 'static + SubscriptionWithTeardown + Drop + Send + Sync
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	/// Create a Subscription for this [Observable]. This action allocates
	/// resources to execute the behavior this [Observable] defines,
	/// essentially creating an instance of it.
	///
	/// The returned [Subscription][Observable::Subscription] can be used to
	/// release the allocated resources and stop the subscription by calling
	/// [unsubscribe][crate::SubscriptionLike::unsubscribe]
	///
	/// ## Subscription Drop Behavior
	///
	/// If a subscription has not been unsubscribed manually, they will always
	/// attempt to unsubscribe themselves on drop.
	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync;
}
