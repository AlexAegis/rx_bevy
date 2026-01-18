use core::{fmt::Debug, ops::AddAssign};

use crate::{WithWorkContextProvider, WithWorkInputOutput, WorkContextProvider};

pub trait ScheduledWork: WithWorkInputOutput + WithWorkContextProvider {
	fn tick(
		&mut self,
		input: Self::Tick,
		context: &mut <Self::WorkContextProvider as WorkContextProvider>::Item<'_>,
	) -> WorkResult;

	/// This hook is called once when the work enters the executor during the
	/// first tick after being scheduled.
	fn on_scheduled_hook(&mut self, tick_input: Self::Tick);
}

#[derive(Debug)]
pub enum WorkResult {
	/// Work that is done will be immediately removed from the scheduler
	Done,
	/// Pending work will be kept in the scheduler
	Pending,
}

impl AddAssign for WorkResult {
	fn add_assign(&mut self, rhs: Self) {
		let change = match self {
			Self::Pending => Some(rhs),
			Self::Done => match rhs {
				Self::Pending => None,
				_ => Some(rhs),
			},
		};

		if let Some(change) = change {
			*self = change;
		}
	}
}
