use core::{fmt::Debug, ops::AddAssign};

use crate::{WithWorkContextProvider, WithWorkInputOutput, WorkContextProvider};

pub trait ScheduledWork: WithWorkInputOutput + WithWorkContextProvider {
	fn tick(
		&mut self,
		input: Self::Tick,
		context: &mut <Self::WorkContextProvider as WorkContextProvider>::Item<'_>,
	) -> WorkResult;

	/// The scheduler should call this immediately when you pass the work into
	/// it, which happens before the first tick can.
	///
	/// TODO: VErify if it even makes sense or just defer to the next first tick
	/// on drain to act as initialize
	///
	/// TODO: ADD A RETURN VALUE AND RETURN IT TO THE USER, BUT ONLY MAKES SENSE WITH THE CONTEXT, BUT THAT CAN'T BE CALLED??
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
