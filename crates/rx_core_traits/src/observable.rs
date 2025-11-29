use crate::{
	Signal, Subscriber, SubscriptionContext, SubscriptionScheduled, UpgradeableObserver,
	WithPrimaryCategory, WithSubscriptionContext,
};

/// # [ObservableOutput]
///
/// Defines the outputs of an [Observable]. Also used for [Operator]s to define
/// the new outputs once the operator is applies.
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
/// associated with this subscription, after which it is always safe to drop
/// regardless of the kind of [Context][crate::SubscriptionContext] used.
///
/// ## [Contexts][crate::SubscriptionContext]
///
/// Since everything is stored in subscription, the unit of execution is the
/// subscription value. But not everything can be stored here: In some
/// environments to react to an observed signal, a reference to something
/// temporary is needed. For example, if you're storing subscriptions in an ECS
/// as a component on an entity, then to release that subscription, you must
/// interact with the ECS. And since subscriptions can only interact with what
/// they contain, that would mean we either store some reference under a lock
/// to it, which would inevitably result in deadlocks, or since actions can only
/// happen when something is pushed, we can also just pass in a context along
/// with every pushed signal.
///
/// This is what contexts are for, to provide temporary references to things
/// that may only live for the instant you're pushing a signal.
///
/// > This is possible because in `rx_bevy` new values cannot be produced
/// > without explicit action, like when a subscription is established, or when
/// > you push a new signal into an observer. But then how can timer-like
/// > Observables work like the IntervalObservable that emits a new value
/// > periodically? The answer is ticking. To signal the passage of time,
/// > a subscription must be "ticked", which will result in moving the internal
/// > clocks of the subscriptions forward, and this action can result in
/// > additional signals, which is why ticking also require a context.
///
/// ## Dropping Subscriptions
///
/// Subscriptions that were not unsubscribed when they are dropped will try to
/// unsubscribe themselves. If you use a
/// [DropUnsafeSubscriptionContext][crate::DropUnsafeSubscriptionContext], one that can't
/// just be created from the subscription itself (like unit `()`), this will
/// result in a panic. But do not worry, such contexts are only ever
/// explicitly used, and are usually used in managed environment where you
/// don't directly handle subscriptions, such as in an ECS where everything
/// is wrapped into components and events.
///
/// > Note that not assigning the subscription to a variable (or assining it to
/// > `let _ =`) will cause it to be immediately dropped, hence `subscribe` is
/// > `#[must_use]`!
pub trait Observable: ObservableOutput + WithSubscriptionContext + WithPrimaryCategory {
	/// The subscription produced by this [Observable]. As this is the only kind
	/// of subscription that is handled directly by users, only here are
	/// subscriptions required to implement [Drop] to ensure resources
	/// are released when the subscription is dropped, and an unsubscribe can
	/// be attempted. This attempt at unsubscribing on drop, if the subscription
	/// wasn't already unsubscribed, can panic if the SubscriptionContext used
	/// is not a [DropSafeSubscriptionContext].
	type Subscription<Destination>: 'static
		+ SubscriptionScheduled<Context = Self::Context>
		+ Drop
		+ Send
		+ Sync
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

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
	/// Depending on the context being used to create the subscription, this
	/// could be completely fine, or result in a panic.
	///
	/// ### Subscription Drop Safety
	///
	/// The context used define if a subscription is safe to drop or not.
	///
	/// #### [DropSafeSubscriptionContext][crate::DropSafeSubscriptionContext]
	///
	/// Simple subscriptions that use a
	/// [DropSafeSubscriptionContext][crate::DropSafeSubscriptionContext] like
	/// `()` are always safe to drop without manually unsubscribing it before.
	/// This is possible because the context can be trivially created during
	/// drop either out of thin air or from the subscription itself.
	///  
	/// #### [DropUnsafeSubscriptionContext][crate::DropUnsafeSubscriptionContext]
	///
	/// Some subscriptions may need to allocate resources by interacting with
	/// another system, for example by spawning entities in an ECS. They are
	/// able to interact with this system, by the reference to this context.
	/// And since you needed to interact with this system to allocate the
	/// resource, you may also need to interact with it to deallocate it, like
	/// despawning an entity. Since this context is not available during drop,
	/// a panic would happen if they get dropped without unsubscribing them
	/// first. In practice, this will not happen as subscriptions are expected
	/// to be completely integrated into the ECS, and a simple `on_remove` hook
	/// can take care of unsubscribing the subscription because it can be
	/// dropped.
	/// This is necessary to ensure memory leaks can't happen and the expected
	/// logic of your pipelines is sound.
	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;
}

/// For usecases where the context is not used at all, some convenience
/// functions are provided
pub trait ObservableWithDefaultDropContext: Observable {
	/// Convenience function that uses the default drop context to `subscribe`
	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe_noctx<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let mut context = Self::Context::create_context_to_unsubscribe_on_drop();
		self.subscribe(destination, &mut context)
	}
}

impl<T> ObservableWithDefaultDropContext for T where T: Observable {}
