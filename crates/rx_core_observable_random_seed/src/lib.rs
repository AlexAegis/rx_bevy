//! Random Seed Observable for `rx_core`

pub use rx_core_common::*;
pub use rx_core_macro_observable_derive::Observable;
use rx_core_subscription_inert::InertSubscription;

use rand::Rng;

/// An observable that emits a random `u64` seed.
#[derive(Observable)]
#[observable(emits = u64, name = "RandomSeedObservable")]
pub struct RandomSeedObservable;

impl ObservableSubscribe for RandomSeedObservable {
    type Subscription = InertSubscription;

    fn subscribe(self, mut observer: Observer<Self>) {
        observer.next(rand::thread_rng().gen::<u64>());
        observer.complete();
        InertSubscription
    }
}
