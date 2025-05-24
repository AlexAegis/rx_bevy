use std::any::Any;

use bevy::{platform::collections::HashMap, prelude::*};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

use super::{SignalKeyPair, SignalTransformer, SignalTransformerPlugin};

/// TODO: Maybe this is unnecessary, instances are stored on entities because they are stateful
#[derive(Resource, Default, Deref, DerefMut)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Resource, Default))]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(
	all(feature = "inspector", feature = "reflect"),
	reflect(InspectorOptions)
)]
pub(crate) struct SignalTransformerRegistry {
	// erased, later reified
	#[deref]
	#[reflect(ignore)]
	pub(crate) transformers: HashMap<SignalKeyPair, Box<dyn Any + Send + Sync + 'static>>,
}

impl SignalTransformerRegistry {
	pub(crate) fn get_transformer<T: SignalTransformer>(&self) -> Option<&T> {
		self.get(&T::signal_key())
			.and_then(|erased_transformer| erased_transformer.downcast_ref::<T>())
	}
}

pub trait SignalTransformerAppExtension {
	fn register_signal_transformer<T: SignalTransformer>(&mut self) -> &mut Self;
}

impl SignalTransformerAppExtension for App {
	fn register_signal_transformer<T: SignalTransformer>(&mut self) -> &mut Self {
		#[cfg(feature = "reflect")]
		self.register_type::<T>();

		self.add_plugins(SignalTransformerPlugin::<T::InputSignal, T::OutputSignal>::default());

		let mut registry = self
			.world_mut()
			.get_resource_or_insert_with(SignalTransformerRegistry::default);

		registry
			.transformers
			.insert(T::signal_key(), Box::new(T::default()));

		self
	}
}
