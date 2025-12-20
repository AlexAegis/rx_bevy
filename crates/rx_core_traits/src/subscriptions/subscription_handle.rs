use std::sync::{Arc, Mutex};

use rx_core_macro_subscription_derive::RxSubscription;

use crate::SubscriptionData;

#[derive(RxSubscription, Default, Clone, Debug)]
#[_rx_core_traits_crate(crate)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl] // It's shared
pub struct SubscriptionHandle {
	#[destination]
	subscription: Arc<Mutex<SubscriptionData>>,
}
