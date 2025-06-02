use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{
	Action, ActionKeyPair, ActionSocket, ActionSystem, ActionSystemFor, Clock, Signal,
	SignalTransformContext, SignalTransformer,
};

use super::{
	ErasedTransformerState, SignalPropagationEvent, SocketActionMap, SocketConnectorSource,
	TransformerOutputCache,
};

#[derive_where(Default)]
pub struct SocketMapperPlugin<FromAction: Action, ToAction: Action> {
	_phantom_data: PhantomData<(FromAction, ToAction)>,
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

/// TODO: This feels like a bottlenect
#[derive(Event)]
struct SocketWriteEvent<A: Action> {
	action: A,
	signal: <A as Action>::Signal,
}

fn propagator() {}

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

	println!(
		"transformless_handler finished {:?} {:?}",
		std::any::type_name::<FromSignal>(),
		std::any::type_name::<ToSignal>()
	);
}

fn transform_handler<C: Clock, T: SignalTransformer>(
	trigger: Trigger<SignalTransformEvent<T::InputSignal>>,
	mut commands: Commands,
	mut erased_transformer_state_query: Query<(
		&mut ErasedTransformerState,
		&mut TransformerOutputCache<T::OutputSignal>,
		// &mut &SocketConnections<A>, // problem that this is erased
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

			//// TODO:  Continue triggering propagation
		}
	};

	println!(
		"transform_handler finished {:?}",
		std::any::type_name::<T>()
	);
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
