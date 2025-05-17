use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, ActionSystem, SignalTransformer, SocketConnector};

/// Displays colored lines between source and target action sockets based on
/// their connectors.
/// TODO: Line color saturation/brightness based on the value in it's range
/// TODO: Different gizmos when a connection points to the same translation vs between different positions/entities
/// TODO: "Bus" display, since an action can have multiple variants, and each have their own signals, either it should be configurable to which ones to display, and when multiple is displayed, they should be side by side relative to the camera
#[derive_where(Default)]
pub struct ActionSignalDebugGizmoPlugin<FromAction, ToAction, Transformer>
where
	FromAction: Action,
	ToAction: Action,
	Transformer:
		SignalTransformer<InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>,
{
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
	_phantom_data_transformer: PhantomData<Transformer>,
}

impl<FromAction, ToAction, Transformer> Plugin
	for ActionSignalDebugGizmoPlugin<FromAction, ToAction, Transformer>
where
	FromAction: Action,
	ToAction: Action,
	Transformer: SignalTransformer<InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>
		+ 'static
		+ Send
		+ Sync,
{
	fn build(&self, app: &mut App) {
		app.add_systems(
			PreUpdate,
			draw_connector_gizmo::<FromAction, ToAction, Transformer>
				.in_set(ActionSystem::Triggered),
		);
	}
}

// TODO: Finish once connectors use relationships
fn draw_connector_gizmo<FromAction, ToAction, Transformer>(
	mut _gizmo: Gizmos,
	query: Query<(
		Entity,
		&SocketConnector<FromAction, ToAction, Transformer>,
		Option<&Name>,
	)>,
) where
	FromAction: Action,
	ToAction: Action,
	Transformer: SignalTransformer<InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>
		+ 'static
		+ Send
		+ Sync,
{
	for (_entity, _connector, _name) in query.iter() {
		// gizmo.line(start, end, color);
	}
}
