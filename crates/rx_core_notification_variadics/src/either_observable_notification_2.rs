use core::marker::PhantomData;

use rx_core_traits::{ObservableOutput, SubscriberNotification};

#[derive(Debug)]
pub enum EitherObservableNotification2<O1, O2>
where
	O1: ObservableOutput,
	O2: ObservableOutput,
{
	O1(SubscriberNotification<O1::Out, O1::OutError>),
	O2(SubscriberNotification<O2::Out, O2::OutError>),
}

pub trait EitherNotificationSelector2<O1, O2>
where
	O1: ObservableOutput,
	O2: ObservableOutput,
{
	type Variant: ObservableOutput;

	fn select(
		notification: SubscriberNotification<
			<Self::Variant as ObservableOutput>::Out,
			<Self::Variant as ObservableOutput>::OutError,
		>,
	) -> EitherObservableNotification2<O1, O2>;
}

pub struct EitherNotificationSelector1Of2<O1, O2> {
	_phantom_data: PhantomData<fn((O1, O2)) -> (O1, O2)>,
}

impl<O1, O2> EitherNotificationSelector2<O1, O2> for EitherNotificationSelector1Of2<O1, O2>
where
	O1: ObservableOutput,
	O2: ObservableOutput,
{
	type Variant = O1;

	fn select(
		notification: SubscriberNotification<
			<Self::Variant as ObservableOutput>::Out,
			<Self::Variant as ObservableOutput>::OutError,
		>,
	) -> EitherObservableNotification2<O1, O2> {
		EitherObservableNotification2::O1(notification)
	}
}

pub struct EitherNotificationSelector2Of2<O1, O2> {
	_phantom_data: PhantomData<fn((O1, O2)) -> (O1, O2)>,
}

impl<O1, O2> EitherNotificationSelector2<O1, O2> for EitherNotificationSelector2Of2<O1, O2>
where
	O1: ObservableOutput,
	O2: ObservableOutput,
{
	type Variant = O2;

	fn select(
		notification: SubscriberNotification<
			<Self::Variant as ObservableOutput>::Out,
			<Self::Variant as ObservableOutput>::OutError,
		>,
	) -> EitherObservableNotification2<O1, O2> {
		EitherObservableNotification2::O2(notification)
	}
}
