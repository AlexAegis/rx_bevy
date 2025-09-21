use rx_bevy_core::{Observable, Subscriber};

use crate::{EitherOut2, EitherOutError2};

#[deprecated = "unused"]
pub enum EitherObservable<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>, InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	O1((O1, Destination)),
	O2((O2, Destination)),
}
