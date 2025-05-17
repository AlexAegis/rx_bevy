use std::{any::TypeId, marker::PhantomData};

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{
	Action, ActionKeyPair, ActionSocket, ActionSystem, ActionSystemFor, Clock, Signal,
	SignalKeyPair, SignalTransformContext, SignalTransformer, SignalTransformerRegistry,
	SignalWriter, SocketConnections,
};

use super::{
	ErasedTransformerState, SignalPropagationEvent, SocketActionMap, SocketConnector,
	SocketConnectorSource, TransformerOutputCache,
};

#[derive_where(Default)]
pub struct SocketMapperPlugin<FromAction: Action, ToAction: Action> {
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
}

impl<FromAction: Action, ToAction: Action> Plugin for SocketMapperPlugin<FromAction, ToAction> {
	fn build(&self, app: &mut App) {
		app.add_event::<SignalTransformEvent<FromAction::Signal>>();

		// TODO: Maybe unnecessary since its handled by observables only
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Map
				.after(ActionSystem::InputSocketWrite)
				.after(ActionSystemFor::<FromAction>::Propagate)
				.before(ActionSystemFor::<ToAction>::Propagate)
				.before(ActionSystem::PropagationDone),
		);
	}
}

/// This internal event notifies others that this socket is ready to be
/// mapped from
#[derive(Event)]
struct SignalTransformEvent<InputSignal: Signal> {
	from_entity: Entity,
	action_key_pair: ActionKeyPair,
	signal: InputSignal,
	last_frame_input_signal: InputSignal,
}

/// Reacts to propagation where there is no transformer set up since the
/// input and output signals match.
fn transformless_handler<C: Clock, FromSignal: Signal, ToSignal: Signal + From<FromSignal>>(
	trigger: Trigger<SignalTransformEvent<FromSignal>>,
	mut commands: Commands,
	erased_transformer_state_query: Query<&ErasedTransformerState>,
	mut output_cache_query: Query<&mut TransformerOutputCache<ToSignal>>,
) {
	let event = trigger.event();

	let has_transformer = erased_transformer_state_query
		.get(trigger.target())
		.map(|t| t.transformer_map.contains_key(&event.action_key_pair))
		.unwrap_or(false);

	if !has_transformer {
		let to_signal = ToSignal::from(event.signal);

		if let Ok(mut e) = output_cache_query.get_mut(trigger.target()) {
			e.transformer_map
				.entry(event.action_key_pair)
				.insert(to_signal);
		}
	}
}

fn transform_handler<C: Clock, T: SignalTransformer>(
	trigger: Trigger<SignalTransformEvent<T::InputSignal>>,
	mut commands: Commands,
	mut erased_transformer_state_query: Query<(
		&mut ErasedTransformerState,
		&mut TransformerOutputCache<T::OutputSignal>,
	)>,
	time: Res<Time<C>>,
) {
	let event = trigger.event();
	if let Ok((mut transformer_state, mut transformer_output_cache)) =
		erased_transformer_state_query.get_mut(trigger.target())
	{
		if let Some(transformer) = transformer_state
			.transformer_map
			.get_mut(&event.action_key_pair)
			.and_then(|a| a.downcast_mut::<T>())
		{
			let to_signal = transformer.transform(
				&event.signal,
				SignalTransformContext::<'_, C, T::InputSignal> {
					time: &time,
					last_frame_input_signal: &event.last_frame_input_signal,
				},
			);

			transformer_output_cache
				.transformer_map
				.entry(event.action_key_pair)
				.insert(to_signal);

			//commands.trigger(Sig);
			//// TODO:  Continue triggering propagation
		}
	};
}

// TODO: CONNECT ME INTO A PLUGIN
/// Triggers transformations on mapper entities where an [SocketActionMap] and
/// a [SocketConnectorSource] exists
fn map_actions_trigger_transformers<FromAction: Action, ToAction: Action>(
	trigger: Trigger<SignalPropagationEvent<FromAction>>,
	mut commands: Commands,
	from_socket_query: Query<&ActionSocket<FromAction>>,
	mut socket_action_map_query: Query<(
		Entity,
		&SocketConnectorSource<FromAction>,
		&SocketActionMap<FromAction, ToAction>,
	)>,
) {
	let Ok((mapper_entity, connector_source, action_map)) =
		socket_action_map_query.get_mut(trigger.target())
	else {
		return;
	};

	let Ok(source_socket) = from_socket_query.get(**connector_source) else {
		return;
	};

	for (_from_action, _to_action, input_signal_container) in
		source_socket.iter_mappable_containers(action_map)
	{
		commands.trigger_targets(
			SignalTransformEvent::<FromAction::Signal> {
				action_key_pair: ActionKeyPair::from_actions::<FromAction, ToAction>(),
				from_entity: mapper_entity,
				signal: input_signal_container.signal,
				last_frame_input_signal: input_signal_container.last_frame_signal,
			},
			mapper_entity,
		);
	}
}
