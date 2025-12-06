use rx_core_traits::TaskId;

#[derive(Default, Debug)]
pub struct TaskIdGenerator {
	current_tick_index: usize,
}

impl TaskIdGenerator {
	pub fn get_next(&mut self) -> TaskId {
		let tick_id: TaskId = self.current_tick_index.into();
		self.current_tick_index += 1;
		tick_id
	}
}
