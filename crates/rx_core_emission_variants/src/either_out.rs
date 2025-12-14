use derive_where::derive_where;
use rx_core_traits::Observable;

#[derive_where(Debug; O1::Out, O2::Out)]
pub enum EitherOut2<O1, O2>
where
	O1: Observable,
	O2: Observable,
{
	O1(O1::Out),
	CompleteO1,
	O2(O2::Out),
	CompleteO2,
}
