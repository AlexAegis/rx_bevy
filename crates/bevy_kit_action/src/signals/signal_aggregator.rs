use std::{
	cmp::Ordering,
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
	fn combine(&self, signals: impl Iterator<Item = S>) -> S;
}

#[derive(Default, Debug, Reflect)]
pub struct SignalAccumulatorOverride;

#[derive(Default, Debug, Reflect)]
pub struct SignalAccumulatorKeepOld;

impl<S: Signal> SignalAggregator<S> for SignalAccumulatorOverride {
	fn combine(&self, signals: impl Iterator<Item = S>) -> S {
		signals.last().unwrap_or_default()
	}
}

impl<S: Signal> SignalAggregator<S> for SignalAccumulatorKeepOld {
	fn combine(&self, mut signals: impl Iterator<Item = S>) -> S {
		signals.next().unwrap_or_default()
	}
}

#[derive(Default, Debug, Reflect)]
pub enum SignalBooleanAggregator {
	#[default]
	Or,
	And,
}

impl SignalAggregator<bool> for SignalBooleanAggregator {
	fn combine(&self, mut signals: impl Iterator<Item = bool>) -> bool {
		match &self {
			SignalBooleanAggregator::And => signals.all(|x| x),
			SignalBooleanAggregator::Or => signals.any(|x| x),
		}
	}
}

#[derive(Default, Debug, Reflect)]
pub enum SignalNumberAggregator {
	#[default]
	Max,
	Min,
	Add,
	Multiply,
}

/// Acts as a not-so-correct Ordering for partially orderable values like f32 since
/// NaN and 0 != -0 would break Ord. But as long as we conveniently ignore this
/// we can get a nice comparator.
fn partial_ord_compare<T: PartialOrd>(a: &T, b: &T) -> Ordering {
	PartialOrd::partial_cmp(a, b).unwrap_or(Ordering::Equal)
}

impl<S: Signal + Add<Output = S> + Mul<Output = S> + PartialOrd> SignalAggregator<S>
	for SignalNumberAggregator
{
	fn combine(&self, signals: impl Iterator<Item = S>) -> S {
		match &self {
			SignalNumberAggregator::Add => signals.fold(S::default(), |a, b| a + b),
			SignalNumberAggregator::Max => signals.max_by(partial_ord_compare).unwrap_or_default(),
			SignalNumberAggregator::Min => signals.min_by(partial_ord_compare).unwrap_or_default(),
			SignalNumberAggregator::Multiply => signals.fold(S::default(), |a, b| a * b),
		}
	}
}

/// TODO: For Vectors, more complex accumulators could be implemented. but for example if an average is needed across all inputs, that cant be done with this style of accumulators
#[derive(Default, Debug, Reflect)]
pub enum SignalVec2Aggregator {
	#[default]
	Set,
}
