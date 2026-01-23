use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, AtomicUsize, Ordering},
	},
	time::Duration,
};

use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_common::SharedSubscription;

mod immediate_work {
	use super::*;

	#[test]
	fn should_execute_nested_immediate_work_in_a_single_update() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let inner_work_has_executed = Arc::new(AtomicBool::default());
		let inner_work_has_executed_work = inner_work_has_executed.clone();

		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		let scheduler_handle = scheduler.handle();
		let scheduler_handle_level_1 = scheduler_handle.clone();
		let scheduler_handle_level_2 = scheduler_handle.clone();
		{
			let mut scheduler = scheduler_handle.lock();

			let cancellation_id_1 = scheduler.generate_cancellation_id();
			let cancellation_id_2 = scheduler.generate_cancellation_id();
			let cancellation_id_3 = scheduler.generate_cancellation_id();
			scheduler.schedule_immediate_work(
				move |_, _| {
					scheduler_handle_level_1.lock().schedule_immediate_work(
						move |_, _| {
							scheduler_handle_level_2.lock().schedule_immediate_work(
								move |_, _| {
									inner_work_has_executed_work.store(true, Ordering::Relaxed);
								},
								cancellation_id_3,
							);
						},
						cancellation_id_2,
					);
				},
				cancellation_id_1,
			);
		};

		assert!(!inner_work_has_executed.load(Ordering::Relaxed));

		app.update();

		assert!(inner_work_has_executed.load(Ordering::Relaxed));
	}
}

mod delayed_work {
	use super::*;

	#[test]
	fn should_execute_nested_delayed_work_in_individual_updates() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let inner_work_has_executed = Arc::new(AtomicBool::default());
		let inner_work_has_executed_work = inner_work_has_executed.clone();

		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		let scheduler_handle = scheduler.handle();
		let scheduler_handle_level_1 = scheduler_handle.clone();
		let scheduler_handle_level_2 = scheduler_handle.clone();
		{
			let mut scheduler = scheduler_handle.lock();

			let cancellation_id_1 = scheduler.generate_cancellation_id();
			let cancellation_id_2 = scheduler.generate_cancellation_id();
			let cancellation_id_3 = scheduler.generate_cancellation_id();
			scheduler.schedule_delayed_work(
				move |_, _| {
					scheduler_handle_level_1.lock().schedule_delayed_work(
						move |_, _| {
							scheduler_handle_level_2.lock().schedule_delayed_work(
								move |_, _| {
									inner_work_has_executed_work.store(true, Ordering::Relaxed);
								},
								Duration::from_millis(10),
								cancellation_id_3,
							);
						},
						Duration::from_millis(10),
						cancellation_id_2,
					);
				},
				Duration::from_millis(10),
				cancellation_id_1,
			);
		};

		assert!(!inner_work_has_executed.load(Ordering::Relaxed));

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(300));
		app.update();
		assert!(!inner_work_has_executed.load(Ordering::Relaxed));

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(300));
		app.update();
		assert!(!inner_work_has_executed.load(Ordering::Relaxed));

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(300));
		app.update();
		assert!(inner_work_has_executed.load(Ordering::Relaxed));
	}
}

mod repeated_work {
	use super::*;

	#[test]
	fn should_be_able_to_schedule_repeated_work_and_execute_it_when_it_rolls_over() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let interval_counter = Arc::new(AtomicUsize::default());
		let interval_counter_work = interval_counter.clone();

		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		let scheduler_handle = scheduler.handle();
		let cancellation_id = {
			let mut scheduler = scheduler_handle.lock();

			let cancellation_id = scheduler.generate_cancellation_id();
			scheduler.schedule_repeated_work(
				move |_, _| {
					interval_counter_work.fetch_add(1, Ordering::Relaxed);
					WorkResult::Pending
				},
				Duration::from_millis(1000),
				false,
				5,
				cancellation_id,
			);

			cancellation_id
		};

		assert_eq!(interval_counter.load(Ordering::Relaxed), 0);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(1500));
		app.update();

		assert_eq!(interval_counter.load(Ordering::Relaxed), 1);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(1500));
		app.update();

		assert_eq!(interval_counter.load(Ordering::Relaxed), 3);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(7000));
		app.update();

		assert_eq!(interval_counter.load(Ordering::Relaxed), 8);

		scheduler_handle.lock().cancel(cancellation_id);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(1000));
		app.update();

		assert_eq!(interval_counter.load(Ordering::Relaxed), 8);
	}

	#[test]
	fn should_be_able_to_start_repeated_work_immediately() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let interval_counter = Arc::new(AtomicUsize::default());
		let interval_counter_work = interval_counter.clone();

		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		let scheduler_handle = scheduler.handle();
		{
			let mut scheduler = scheduler_handle.lock();

			let cancellation_id = scheduler.generate_cancellation_id();
			scheduler.schedule_repeated_work(
				move |_, _| {
					interval_counter_work.fetch_add(1, Ordering::Relaxed);
					WorkResult::Pending
				},
				Duration::from_millis(1000),
				true,
				5,
				cancellation_id,
			);
		};

		assert_eq!(interval_counter.load(Ordering::Relaxed), 0);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(1));
		app.update();

		assert_eq!(interval_counter.load(Ordering::Relaxed), 1);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(999));
		app.update();

		assert_eq!(interval_counter.load(Ordering::Relaxed), 2);
	}
}

mod continuous_work {
	use super::*;

	#[test]
	fn should_execute_continuous_work_on_each_update() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let execution_counter = Arc::new(AtomicUsize::default());
		let execution_counter_work = execution_counter.clone();

		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		let scheduler_handle = scheduler.handle();
		let cancellation_id = {
			let mut scheduler = scheduler_handle.lock();

			let cancellation_id = scheduler.generate_cancellation_id();
			scheduler.schedule_continuous_work(
				move |_, _| {
					execution_counter_work.fetch_add(1, Ordering::Relaxed);
					WorkResult::Pending
				},
				cancellation_id,
			);

			cancellation_id
		};

		assert_eq!(execution_counter.load(Ordering::Relaxed), 0);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(1));
		app.update();

		assert_eq!(execution_counter.load(Ordering::Relaxed), 1);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(1));
		app.update();

		assert_eq!(execution_counter.load(Ordering::Relaxed), 2);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(1));
		app.update();

		assert_eq!(execution_counter.load(Ordering::Relaxed), 3);

		scheduler_handle.lock().cancel(cancellation_id);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_millis(1));
		app.update();

		assert_eq!(execution_counter.load(Ordering::Relaxed), 3);
	}
}

mod invoked {
	use super::*;

	#[test]
	fn should_be_possible_to_schedule_and_invoke_work_manually() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let invoked_work_was_called = Arc::new(AtomicBool::new(false));

		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		let scheduler_handle = scheduler.handle();
		let invoke_id = {
			let mut scheduler_lock = scheduler_handle.lock();
			let invoke_id = scheduler_lock.generate_invoke_id();
			let invoked_work_was_called_clone = invoked_work_was_called.clone();
			scheduler_lock.schedule_invoked_work(
				<RxBevyScheduler as Scheduler>::InvokedWorkFactory::new(move |_, _| {
					invoked_work_was_called_clone.store(true, Ordering::Relaxed);
					WorkResult::Done
				}),
				invoke_id,
			);
			invoke_id
		};

		app.add_systems(Update, move |scheduler: RxSchedule<Update, Virtual>| {
			scheduler.handle().lock().invoke(invoke_id);
		});

		app.update();

		assert!(invoked_work_was_called.load(Ordering::Relaxed));
	}

	#[test]
	fn should_be_possible_to_cancel_an_invoked_work_in_the_same_frame() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let invoked_work_was_called = Arc::new(AtomicBool::new(false));

		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		let scheduler_handle = scheduler.handle();
		let invoke_id = {
			let mut scheduler_lock = scheduler_handle.lock();
			let invoke_id = scheduler_lock.generate_invoke_id();
			let invoked_work_was_called_clone = invoked_work_was_called.clone();
			scheduler_lock.schedule_invoked_work(
				<RxBevyScheduler as Scheduler>::InvokedWorkFactory::new(move |_, _| {
					invoked_work_was_called_clone.store(true, Ordering::Relaxed);
					WorkResult::Done
				}),
				invoke_id,
			);
			invoke_id
		};

		app.add_systems(PreUpdate, move |scheduler: RxSchedule<Update, Virtual>| {
			scheduler.handle().lock().cancel_invoked(invoke_id);
		});

		app.add_systems(Update, move |scheduler: RxSchedule<Update, Virtual>| {
			scheduler.handle().lock().invoke(invoke_id);
		});

		app.update();

		assert!(!invoked_work_was_called.load(Ordering::Relaxed));
	}
}

mod cleanup_on_exit {
	use super::*;

	#[test]
	fn rx_plugin_unsubscribes_all_subscriptions_on_app_exit() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins(RxPlugin);

		let subscription = SharedSubscription::default();
		let subscription_clone = subscription.clone();
		assert!(!subscription_clone.is_closed());

		app.world_mut()
			.spawn(SubscriptionComponent::new(subscription));

		app.update();

		app.world_mut().write_message(AppExit::Success);

		app.update();

		assert!(
			subscription_clone.is_closed(),
			"subscription should be unsubscribed when the app exits"
		);
	}
}
