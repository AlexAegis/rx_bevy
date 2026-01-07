use bevy_ecs::{
	entity::Entity,
	system::{Commands, Query, SystemParam},
};
use disqualified::ShortName;
use rx_core_common::{Never, Signal, UpgradeableObserver};
use rx_core_macro_observable_derive::RxObservable;

use crate::{CommandSubscribeExtension, ObservableOutputs, SubscribeError};

/// An alternative interface to subscribe to observables, offering eager
/// checks.
#[derive(RxObservable, SystemParam)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct ObservableQuery<'w, 's, Out, OutError = Never>
where
	Out: Signal,
	OutError: Signal,
{
	commands: Commands<'w, 's>,
	observable: Query<'w, 's, &'static ObservableOutputs<Out, OutError>>,
}

impl<'w, 's, Out, OutError> ObservableQuery<'w, 's, Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	/// Attempts a checked subscription to an observable entity, which if does
	/// not contain an observable with outputs`Out` and `OutError` will return
	/// an error **immediately**.
	///
	/// Since the check happens immediately, observables spawned in the same
	/// system will not be found. In that case, use `Commands::subscribe`
	/// directly, as that will automatically be retried in the next frame
	/// if the Observable wasn't available immediately.
	pub fn try_subscribe_to(
		&mut self,
		observable_entity: Entity,
		destination: impl 'static + UpgradeableObserver<In = Out, InError = OutError>,
	) -> Result<Entity, SubscribeError> {
		if self.observable.contains(observable_entity) {
			Ok(self.commands.subscribe::<_>(observable_entity, destination))
		} else {
			Err(SubscribeError::NotAnObservable(
				format!(
					"{{unknown observable}}<{}, {}>",
					ShortName::of::<Out>(),
					ShortName::of::<OutError>()
				),
				observable_entity,
			))
		}
	}
}
