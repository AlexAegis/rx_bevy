use super::Observable;

pub trait ObservablePipe<Op> {
	fn with_operator(operator: Op);
}
