use std::marker::PhantomData;

use crate::{IdentitySubscriber, ObservableOutput, ObserverInput, Operator, Subscriber};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// # [IdentityOperator]
///
/// The [IdentityOperator] does nothing. It's only purpose is to let you
/// easily define input types for a [CompositeOperator]
#[derive(Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IdentityOperator<In, InError> {
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Default for IdentityOperator<In, InError> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Clone for IdentityOperator<In, InError> {
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ObservableOutput for IdentityOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError> ObserverInput for IdentityOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> Operator for IdentityOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		IdentitySubscriber<Destination>;

	#[inline]
	fn operator_subscribe<
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		IdentitySubscriber::new(destination)
	}
}
