use std::{
	cmp::Ordering,
	ops::{Add, Div, Mul},
};

use num_traits::{FromPrimitive, One, Zero};

use crate::{Signal, SignalAggregator};

#[cfg(feature = "reflect")]
use bevy::prelude::*;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub enum SignalNumberAggregator {
	#[default]
	Max,
	Min,
	Add,
	Multiply,
	Average,
	Median,
}

/// Acts as a not-so-correct Ordering for partially orderable values like f32 since
/// NaN and 0 != -0 would break Ord. But as long as we conveniently ignore this
/// we can get a nice comparator.
fn partial_ord_compare<T: PartialOrd>(a: &T, b: &T) -> Ordering {
	PartialOrd::partial_cmp(a, b).unwrap_or(Ordering::Equal)
}

impl<
	S: Signal
		+ Add<Output = S>
		+ Zero
		+ Mul<Output = S>
		+ One
		+ Div<Output = S>
		+ FromPrimitive
		+ PartialOrd,
> SignalAggregator<S> for SignalNumberAggregator
{
	fn combine(&self, signals: impl Iterator<Item = S>) -> S {
		match &self {
			SignalNumberAggregator::Add => signals.fold(S::zero(), |a, b| a + b),
			SignalNumberAggregator::Max => signals.max_by(partial_ord_compare).unwrap_or_default(),
			SignalNumberAggregator::Min => signals.min_by(partial_ord_compare).unwrap_or_default(),
			SignalNumberAggregator::Multiply => signals.fold(S::one(), |a, b| a * b),
			SignalNumberAggregator::Average => {
				let (sum, count) = signals.fold((S::zero(), S::zero()), |(sum, count), x| {
					(sum + x, count + S::one())
				});
				if count == S::zero() {
					S::zero()
				} else {
					sum / count
				}
			}
			SignalNumberAggregator::Median => {
				let mut values: Vec<S> = signals.collect();
				values.sort_by(partial_ord_compare);
				if values.is_empty() {
					S::zero()
				} else if values.len() % 2 == 0 {
					(values[values.len() / 2 - 1] + values[values.len() / 2])
						/ S::from_u8(2).unwrap()
				} else {
					values[values.len() / 2]
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_max_aggregator() {
		let aggregator = SignalNumberAggregator::Max;
		let signals = vec![1.0, 5.0, 3.0, 2.0];
		assert_eq!(aggregator.combine(signals.into_iter()), 5.0);
	}

	#[test]
	fn test_min_aggregator() {
		let aggregator = SignalNumberAggregator::Min;
		let signals = vec![1.0, 5.0, 3.0, 2.0];
		assert_eq!(aggregator.combine(signals.into_iter()), 1.0);
	}

	#[test]
	fn test_add_aggregator() {
		let aggregator = SignalNumberAggregator::Add;
		let signals = vec![1.0, 2.0, 3.0];
		assert_eq!(aggregator.combine(signals.into_iter()), 6.0);
	}

	#[test]
	fn test_multiply_aggregator() {
		let aggregator = SignalNumberAggregator::Multiply;
		let signals = vec![2.0, 3.0, 4.0];
		assert_eq!(aggregator.combine(signals.into_iter()), 24.0);
	}

	#[test]
	fn test_average_aggregator() {
		let aggregator = SignalNumberAggregator::Average;
		let signals = vec![2.0, 4.0, 6.0];
		assert_eq!(aggregator.combine(signals.into_iter()), 4.0);
	}

	#[test]
	fn test_median_aggregator_odd() {
		let aggregator = SignalNumberAggregator::Median;
		let signals = vec![1.0, 5.0, 3.0];
		assert_eq!(aggregator.combine(signals.into_iter()), 3.0);
	}

	#[test]
	fn test_median_aggregator_even() {
		let aggregator = SignalNumberAggregator::Median;
		let signals = vec![1.0, 2.0, 3.0, 4.0];
		assert_eq!(aggregator.combine(signals.into_iter()), 2.5);
	}

	#[test]
	fn test_empty_signals() {
		let signals: Vec<f64> = vec![];
		assert_eq!(
			SignalNumberAggregator::Max.combine(signals.clone().into_iter()),
			0.0
		);
		assert_eq!(
			SignalNumberAggregator::Min.combine(signals.clone().into_iter()),
			0.0
		);
		assert_eq!(
			SignalNumberAggregator::Add.combine(signals.clone().into_iter()),
			0.0
		);
		assert_eq!(
			SignalNumberAggregator::Multiply.combine(signals.clone().into_iter()),
			1.0
		);
		assert_eq!(
			SignalNumberAggregator::Average.combine(signals.clone().into_iter()),
			0.0
		);
		assert_eq!(
			SignalNumberAggregator::Median.combine(signals.into_iter()),
			0.0
		);
	}
}
