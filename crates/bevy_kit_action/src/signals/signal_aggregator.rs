use std::{
	fmt::Debug,
	ops::{Add, Mul},
};

use bevy::{
	prelude::*,
	reflect::{GetTypeRegistration, Typed},
};

use super::Signal;

pub trait SignalAggregator<S: Signal>:
	Default + Debug + Reflect + GetTypeRegistration + Typed + FromReflect + Send + Sync + 'static
{
	fn combine(&self, accumulator: S, next: S) -> S;
}

#[derive(Default, Debug, Reflect)]
pub struct SignalAccumulatorOverride;

#[derive(Default, Debug, Reflect)]
pub struct SignalAccumulatorKeepOld;

impl<S: Signal> SignalAggregator<S> for SignalAccumulatorOverride {
	fn combine(&self, _accumulator: S, next: S) -> S {
		next
	}
}

impl<S: Signal> SignalAggregator<S> for SignalAccumulatorKeepOld {
	fn combine(&self, accumulator: S, _next: S) -> S {
		accumulator
	}
}

#[derive(Default, Debug, Reflect)]
pub enum SignalAccumulatorBool {
	#[default]
	Or,
	And,
}

impl SignalAggregator<bool> for SignalAccumulatorBool {
	fn combine(&self, accumulator: bool, next: bool) -> bool {
		match &self {
			SignalAccumulatorBool::And => accumulator && next,
			SignalAccumulatorBool::Or => accumulator || next,
		}
	}
}

#[derive(Default, Debug, Reflect)]
pub enum SignalAccumulatorLinear {
	#[default]
	Max,
	Min,
	Add,
	Multiply,
}

impl<S: Signal + Add<Output = S> + Mul<Output = S> + PartialOrd> SignalAggregator<S>
	for SignalAccumulatorLinear
{
	fn combine(&self, accumulator: S, next: S) -> S {
		match &self {
			SignalAccumulatorLinear::Add => accumulator + next,
			SignalAccumulatorLinear::Max => {
				if accumulator > next {
					accumulator
				} else {
					next
				}
			}
			SignalAccumulatorLinear::Min => {
				if accumulator < next {
					accumulator
				} else {
					next
				}
			}
			SignalAccumulatorLinear::Multiply => accumulator * next,
		}
	}
}

/// TODO: For Vectors, more complex accumulators could be implemented. but for example if an average is needed across all inputs, that cant be done with this style of accumulators
#[derive(Default, Debug, Reflect)]
pub enum SignalAccumulatorVec {
	#[default]
	Set,
}
