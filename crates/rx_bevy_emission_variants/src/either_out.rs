use rx_bevy_core::Observable;

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
/*
impl<O1, O2> Debug for EitherOut2<O1, O2> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"{} {{ is_closed: {} }}",
			short_type_name::short_type_name::<Self>(),
			self.is_closed(),
		))
	}
}
*/
