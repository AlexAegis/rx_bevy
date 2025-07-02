use rx_bevy_observable::Observable;

#[derive(Debug)]
pub enum EitherOut2<O1, O2>
where
	O1: Observable,
	O2: Observable,
{
	/// The completion signal is also forwarded through the output channel
	CompleteO1,
	O1(O1::Out),
	CompleteO2,
	O2(O2::Out),
}
