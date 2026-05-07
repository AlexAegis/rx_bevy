use std::time::Duration;

use rx_core_common::{SchedulerHandle, WorkExecutor};
use rx_core_macro_executor_derive::RxExecutor;
use rx_core_scheduler_ticking::{Tick, TickingSchedulerExecutor};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::{TokioScheduler, UnitContext, UnitContextItem};

type InnerExecutor = TickingSchedulerExecutor<TokioScheduler, UnitContext>;

/// Default tick interval for continuous work. 64hz
const DEFAULT_TICK_INTERVAL: Duration = Duration::from_micros(15625);

/// Autonomous executor driven by the tokio runtime.
///
/// Unlike the `TickingSchedulerExecutor` which must be manually
/// ticked from the outside, this executor spawns a background
/// tokio task that drives all scheduled work independently.
///
/// Continuous work (that in a ticking executor runs every tick)
/// is throttled to the configurable `tick_interval`.
///
/// # Usage
///
/// ```no_run
/// use std::time::Duration;
/// use rx_core_scheduler_tokio::TokioExecutor;
/// use rx_core_common::WorkExecutor;
///
/// #[tokio::main]
/// async fn main() {
///     let mut executor = TokioExecutor::builder()
///         .tick_interval(Duration::from_millis(16))
///         .build();
///
///     let scheduler = executor.get_scheduler_handle();
///     executor.start();
///
///     // Use `scheduler` with observables and operators...
///
///     executor.stop().await;
/// }
/// ```
#[derive(RxExecutor)]
#[rx_context(UnitContext)]
#[rx_tick(Tick)]
#[rx_scheduler(TokioScheduler)]
pub struct TokioExecutor {
	#[scheduler_handle]
	scheduler_handle: SchedulerHandle<TokioScheduler>,
	inner: Option<InnerExecutor>,
	cancel_token: CancellationToken,
	task_handle: Option<JoinHandle<InnerExecutor>>,
	tick_interval: Duration,
}

impl TokioExecutor {
	/// Creates a new `TokioExecutor` with the default tick
	/// interval of 10ms.
	pub fn new() -> Self {
		let inner = TickingSchedulerExecutor::new(TokioScheduler::default());
		let scheduler_handle = inner.get_scheduler_handle();

		Self {
			scheduler_handle,
			inner: Some(inner),
			cancel_token: CancellationToken::new(),
			task_handle: None,
			tick_interval: DEFAULT_TICK_INTERVAL,
		}
	}

	/// Returns a builder for configuring the executor.
	pub fn builder() -> TokioExecutorBuilder {
		TokioExecutorBuilder::default()
	}

	/// Returns `true` when the background task is running.
	pub fn is_started(&self) -> bool {
		self.task_handle.is_some()
	}

	/// Starts the background executor task.
	///
	/// Must be called from within a tokio runtime context.
	///
	/// # Panics
	///
	/// Panics if the executor is already started.
	pub fn start(&mut self) {
		let inner = self
			.inner
			.take()
			.expect("TokioExecutor is already started!");

		let cancel_token = self.cancel_token.clone();
		let tick_interval = self.tick_interval;

		self.task_handle = Some(tokio::spawn(async move {
			run_executor_loop(inner, tick_interval, cancel_token).await
		}));
	}

	/// Stops the background executor task and waits for it to
	/// finish. After stopping, the executor can be started again.
	pub async fn stop(&mut self) {
		self.cancel_token.cancel();
		if let Some(handle) = self.task_handle.take()
			&& let Ok(inner) = handle.await
		{
			self.inner = Some(inner);
		}
		self.cancel_token = CancellationToken::new();
	}
}

async fn run_executor_loop(
	mut inner: InnerExecutor,
	tick_interval: Duration,
	cancel_token: CancellationToken,
) -> InnerExecutor {
	let start = tokio::time::Instant::now();
	let mut interval = tokio::time::interval(tick_interval);

	loop {
		tokio::select! {
			_ = interval.tick() => {
				let tick = Tick::new(start.elapsed());
				let mut ctx = UnitContextItem;
				inner.tick_to(tick, &mut ctx);
			}
			() = cancel_token.cancelled() => {
				return inner;
			}
		}
	}
}

impl Default for TokioExecutor {
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for TokioExecutor {
	fn drop(&mut self) {
		self.cancel_token.cancel();
	}
}

/// Builder for configuring a [`TokioExecutor`].
pub struct TokioExecutorBuilder {
	tick_interval: Duration,
}

impl Default for TokioExecutorBuilder {
	fn default() -> Self {
		Self {
			tick_interval: DEFAULT_TICK_INTERVAL,
		}
	}
}

impl TokioExecutorBuilder {
	/// Sets the interval at which the executor ticks.
	///
	/// This controls the granularity of scheduled work execution
	/// and the rate of continuous work. Defaults to 10ms.
	///
	/// Lower intervals mean more precise timing but higher CPU
	/// usage. Higher intervals reduce CPU usage but scheduled
	/// work may execute later than expected.
	pub fn tick_interval(mut self, interval: Duration) -> Self {
		self.tick_interval = interval;
		self
	}

	/// Builds the [`TokioExecutor`].
	pub fn build(self) -> TokioExecutor {
		let inner = TickingSchedulerExecutor::new(TokioScheduler::default());
		let scheduler_handle = inner.get_scheduler_handle();

		TokioExecutor {
			scheduler_handle,
			inner: Some(inner),
			cancel_token: CancellationToken::new(),
			task_handle: None,
			tick_interval: self.tick_interval,
		}
	}
}
