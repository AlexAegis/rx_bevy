use std::{
	fmt::Debug,
	sync::{Arc, Mutex, MutexGuard},
};

use derive_where::derive_where;
use rx_core_common::{Never, Signal};

use crate::NotificationCollector;

#[derive_where(Clone, Default)]
#[derive(Debug)]
pub struct MultiRoundNotificationCollector<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	shared_multi_round_notification_collector:
		Arc<Mutex<MultiRoundNotificationCollectorState<In, InError>>>,
}

impl<In, InError> MultiRoundNotificationCollector<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn lock(&self) -> MutexGuard<'_, MultiRoundNotificationCollectorState<In, InError>> {
		self.shared_multi_round_notification_collector
			.lock()
			.unwrap_or_else(|p| p.into_inner())
	}
}

#[derive_where(Default)]
pub struct MultiRoundNotificationCollectorState<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	rounds: Vec<NotificationCollector<In, InError>>,
}

impl<In, InError> MultiRoundNotificationCollectorState<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	pub fn get_round(&mut self, round: usize) -> NotificationCollector<In, InError> {
		if let Some(existing) = self.rounds.get(round) {
			return existing.clone();
		}
		let collector = NotificationCollector::default();
		self.rounds.insert(round, collector.clone());
		collector
	}

	#[inline]
	pub fn has_round(&self, round: usize) -> bool {
		self.rounds.get(round).is_some()
	}
}

impl<In, InError> Debug for MultiRoundNotificationCollectorState<In, InError>
where
	In: Signal + Debug,
	InError: Signal + Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "MultiRoundNotificationCollectorState:")?;
		for (round, collector) in self.rounds.iter().enumerate() {
			writeln!(f, "- {round}:\t{:?}", collector.lock())?;
		}
		Ok(())
	}
}
