use bevy::prelude::*;

// #[cfg(feature = "serialize")]
// use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

#[derive(Component, Default, Clone, Debug)]
#[require(Name::new("KeyboardActionSink"))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component, Default))]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(
	all(feature = "inspector", feature = "reflect"),
	reflect(InspectorOptions)
)]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(
// 	all(feature = "serialize", feature = "reflect"),
// 	reflect(Serialize, Deserialize)
// )]
pub struct KeyboardActionSink;
