use rx_core_common::Signal;

use crate::observable::JustObservable;

pub fn just<Out>(value: Out) -> JustObservable<Out>
where
	Out: Signal + Clone,
{
	JustObservable::new(value)
}
