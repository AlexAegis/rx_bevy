use crate::SignalAggregator;

#[cfg(feature = "reflect")]
use bevy::prelude::*;

// #[cfg(feature = "serialize")]
// use serde::{Deserialize, Serialize};

/// Resolves a single boolean from multiple ones
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(
// 	all(feature = "serialize", feature = "reflect"),
// 	reflect(Serialize, Deserialize)
// )]
pub enum SignalBooleanAggregator {
	/// `true` when any of the input signals are `true`
	#[default]
	Or,
	/// `true` when all of the input signals are `true`
	And,
	/// `true` when not all of the input signals are `true`
	Nand,
	/// `true` when none of the input signals are `true`
	Nor,
	/// `true` when at least half of input signals are `true`
	Majority,
}

impl SignalAggregator<bool> for SignalBooleanAggregator {
	fn combine(&self, mut signals: impl Iterator<Item = bool>) -> bool {
		match &self {
			SignalBooleanAggregator::And => signals.all(|x| x),
			SignalBooleanAggregator::Or => signals.any(|x| x),
			SignalBooleanAggregator::Nand => !signals.all(|x| x),
			SignalBooleanAggregator::Nor => !signals.any(|x| x),
			SignalBooleanAggregator::Majority => {
				let (count, total) =
					signals.fold((0, 0), |(count, total), x| (count + (x as u32), total + 1));
				count * 2 > total
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_and() {
		let aggregator = SignalBooleanAggregator::And;
		assert!(aggregator.combine([true, true].into_iter()));
		assert!(!aggregator.combine([true, false].into_iter()));
		assert!(!aggregator.combine([false, false].into_iter()));
		assert!(aggregator.combine([true, true, true].into_iter()));
		assert!(!aggregator.combine([true, true, false].into_iter()));
	}

	#[test]
	fn test_or() {
		let aggregator = SignalBooleanAggregator::Or;
		assert!(aggregator.combine([true, true].into_iter()));
		assert!(aggregator.combine([true, false].into_iter()));
		assert!(!aggregator.combine([false, false].into_iter()));
		assert!(!aggregator.combine([false, false, false].into_iter()));
		assert!(aggregator.combine([false, true, false].into_iter()));
	}

	#[test]
	fn test_nand() {
		let aggregator = SignalBooleanAggregator::Nand;
		assert!(!aggregator.combine([true, true].into_iter()));
		assert!(aggregator.combine([true, false].into_iter()));
		assert!(aggregator.combine([false, false].into_iter()));
		assert!(!aggregator.combine([true, true, true].into_iter()));
		assert!(aggregator.combine([true, true, false].into_iter()));
	}

	#[test]
	fn test_nor() {
		let aggregator = SignalBooleanAggregator::Nor;
		assert!(!aggregator.combine([true, true].into_iter()));
		assert!(!aggregator.combine([true, false].into_iter()));
		assert!(aggregator.combine([false, false].into_iter()));
		assert!(aggregator.combine([false, false, false].into_iter()));
		assert!(!aggregator.combine([false, true, false].into_iter()));
	}

	#[test]
	fn test_majority() {
		let aggregator = SignalBooleanAggregator::Majority;
		assert!(aggregator.combine([true, true].into_iter()));
		assert!(!aggregator.combine([true, false].into_iter()));
		assert!(aggregator.combine([true, true, false].into_iter()));
		assert!(!aggregator.combine([false, false, true].into_iter()));
		assert!(aggregator.combine([true, true, true, false].into_iter()));
	}
}
