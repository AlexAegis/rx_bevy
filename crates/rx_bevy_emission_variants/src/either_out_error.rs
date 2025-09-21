use rx_bevy_core::Observable;

#[derive(Debug)]
pub enum EitherOutError2<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	O1Error(O1::OutError),
	O2Error(O2::OutError),
}
