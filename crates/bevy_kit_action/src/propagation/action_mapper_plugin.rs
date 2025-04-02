use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, ActionSocket, ActionSystem, ActionSystemFor, SignalConverter, SocketInput};

use super::ActionMap;

/// TODO: Maybe there could be a mutually exclusive way of setting up mapping between two actions, one is this HashMap based, and the other is just From<> impl based and would be faster and simpler but not configurable at runtime. Or it could be the default value for action pairs where it's implemented
/// Contains and executes activation between action contexts
#[derive_where(Default)]
pub struct ActionMapPlugin<FromAction, ToAction, Converter>
where
	FromAction: Action,
	ToAction: Action,
	Converter: SignalConverter<FromAction::Signal, ToAction::Signal>,
{
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
	_phantom_data_to_converter: PhantomData<Converter>,
}

impl<FromAction, ToAction, Converter> Plugin for ActionMapPlugin<FromAction, ToAction, Converter>
where
	FromAction: Action,
	ToAction: Action,
	Converter: SignalConverter<FromAction::Signal, ToAction::Signal> + 'static,
{
	fn build(&self, app: &mut App) {
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Map
				.after(ActionSystemFor::<FromAction>::Map)
				.after(ActionSystem::Input)
				.before(ActionSystem::Mapped),
		);

		// Make sure there is a resource of the converter to use it as the global fallback option
		app.init_resource::<Converter>();

		// Actions are triggered backwards compared to mapping
		// TODO: Does it matter? Which is better? This is kinda like bubbling. Should it be a crate feature?
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Trigger.before(ActionSystemFor::<FromAction>::Trigger),
		);

		// The mapping system is running in the ToActions Map set as the action
		// it maps from is either created by a device, or manually entered
		app.add_systems(
			PreUpdate,
			map_actions::<FromAction, ToAction, Converter>.in_set(ActionSystemFor::<ToAction>::Map),
		);
	}
}

fn map_actions<FromAction, ToAction, Converter>(
	mut action_socket_query: Query<(
		&ActionMap<FromAction, ToAction>,
		&ActionSocket<FromAction>, // This shouldn't care about how it's stored as long as its mappable data
		&mut ActionSocket<ToAction>, // This shouldn't care about how it's stored as long as its mappable data
		Option<&Converter>,
	)>,
	converter_fallback: Option<Res<Converter>>,
) where
	FromAction: Action,
	ToAction: Action,
	Converter: SignalConverter<FromAction::Signal, ToAction::Signal> + 'static,
{
	for (action_map, from_socket, mut to_socket, converter_override) in
		action_socket_query.iter_mut()
	{
		let Some(converter) = converter_override.or(converter_fallback.as_deref()) else {
			// TODO(bevy-0.16): Could be a system error! Although this should not happen, what if something removes the resource after plugin init?
			error!(
				"Can't find applicable {} signal converter from {} to {}",
				std::any::type_name::<Converter>(),
				std::any::type_name::<FromAction::Signal>(),
				std::any::type_name::<ToAction::Signal>()
			);
			continue;
		};

		for (from_action, from_action_signal) in from_socket.iter() {
			if let Some(to_action) = action_map.get(from_action) {
				// TODO: What about last frame's data? Hardcode it to get a delta, or implement some kind of buffer where you can store whatever
				let converted_signal = converter.convert(from_action_signal);
				to_socket.write(to_action, converted_signal);
			}
		}
	}
}
