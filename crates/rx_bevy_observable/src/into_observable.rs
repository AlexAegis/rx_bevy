use crate::ObservableOutput;

pub trait IntoObservable: ObservableOutput {}

impl<T> IntoObservable for T where T: IntoObservable {}
