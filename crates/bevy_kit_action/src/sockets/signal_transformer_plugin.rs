use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, SignalBuffer, SignalTransformer, SocketConnector};

use super::ActionSocket;
/*
pub trait RegisterSignalTransformer {
	fn register_signal_transformer<Transformer: SignalTransformer + 'static + Send + Sync>(
		&mut self,
	);
}

impl RegisterSignalTransformer for App {
	fn register_signal_transformer<Transformer: SignalTransformer + 'static + Send + Sync>(
		&mut self,
	) {
		self.add_plugins(SignalTransformerPlugin::<Transformer>::default());
	}
}
*/
/*
#[derive_where(Default)]
pub struct SignalTransformerBufferPlugin<FromAction, ToAction, Transformer>
where
	FromAction: Action,
	ToAction: Action,
	Transformer: SignalTransformer<InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>
		+ 'static
		+ Send
		+ Sync,
{
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
	_phantom_data_transformer: PhantomData<Transformer>,
}

impl<FromAction, ToAction, Transformer> Plugin
	for SignalTransformerBufferPlugin<FromAction, ToAction, Transformer>
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
			write_into_transformer_buffers::<FromAction, ToAction, Transformer>,
		);
	}
}
*/
