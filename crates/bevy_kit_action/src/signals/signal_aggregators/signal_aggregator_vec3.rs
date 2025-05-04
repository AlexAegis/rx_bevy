use bevy::math::Vec3;

use crate::SignalAggregator;

#[cfg(feature = "reflect")]
use bevy::prelude::*;

// #[cfg(feature = "serialize")]
// use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(
// 	all(feature = "serialize", feature = "reflect"),
// 	reflect(Serialize, Deserialize)
// )]
pub enum SignalAggregatorVec3 {
	#[default]
	Add,
	Multiply,
	First,
	Last,
}

impl SignalAggregator<Vec3> for SignalAggregatorVec3 {
	fn combine(&self, mut signals: impl Iterator<Item = Vec3>) -> Vec3 {
		match &self {
			SignalAggregatorVec3::Add => signals.fold(Vec3::ZERO, |acc, v| acc + v),
			SignalAggregatorVec3::Multiply => signals.fold(Vec3::ONE, |acc, v| acc * v),
			SignalAggregatorVec3::First => signals.next().unwrap_or(Vec3::ZERO),
			SignalAggregatorVec3::Last => signals.last().unwrap_or(Vec3::ZERO),
		}
	}
}
