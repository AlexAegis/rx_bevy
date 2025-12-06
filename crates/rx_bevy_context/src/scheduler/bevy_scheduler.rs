use std::marker::PhantomData;

use rx_bevy_common::Clock;
use rx_core_scheduler_ticking::TickingScheduler;
use rx_core_traits::{Tick, WithTaskInputOutput};

use crate::RxBevyContext;

pub struct BevyRxScheduler<C>
where
	C: Clock,
{
	_ticking_scheduler: TickingScheduler<RxBevyContext>,
	_phantom_data: PhantomData<C>,
}

impl<C> WithTaskInputOutput for BevyRxScheduler<C>
where
	C: Clock,
{
	type TickInput = Tick;
	type TaskError = ();
	type ContextProvider = RxBevyContext;
}
