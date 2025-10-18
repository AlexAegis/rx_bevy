use derive_where::derive_where;
use rx_core_traits::Observable;

#[derive_where(Debug; O1::OutError, O2::OutError)]
pub enum EitherOutError2<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	O1Error(O1::OutError),
	O2Error(O2::OutError),
}
