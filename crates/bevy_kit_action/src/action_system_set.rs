use std::marker::PhantomData;

use super::Action;
use bevy::ecs::schedule::SystemSet;
use derive_where::derive_where;

/// # System ordering
///
/// TODO: DON'T EVEN RUN THE SUB GRAPHS OF THE CONNECTOR-SOCKET GRAPH WHERE NO CHANGE HAS HAPPENED, mapping wise, BUTsill process internal operations (to transform) that can retrigger its traversal through the graph, as an optimiziation to not process large graphs all the time every frame, just when something happens. except the root input node, and maybe an api to mark something as changed, so that it will definitely re-run this frame. transforms set the changed flag if a change in output had occured to allow for releases style signals work.the graph should be able to return if it's all settled.
/// TODO: THERE SHOULD BE A FLAG IN EACH COMPONENT component (for every entity?) or resource with that generic (whole branch skip.) This changed flag wil lbe toggled in a dedicated systemset, where you have to check for a type if the previous 2 frames were the same, then this frame, skip
/// TODO: Review after implementation if this still holds
///
/// ## Example mapping:
///
/// `K => A`; Keyboard input is mapped to action A
/// `G => A`; Gamepad input is also mapped to action A
/// `A => B`; A is then mapped to a sub action B
///
/// ## System order:
///
/// 1. `ActionSystem::Reset`: Move all [ActionState][crate::ActionState]s to
///    the previous frames [ActionState][crate::ActionState]
/// 2. `InputSystem`: Bevy's InputSystem is ran, collecting keyboard, gamepad,
///    input
/// 2. `ActionSystem::Input`: Collect device inputs into its own
///    [ActionState][crate::ActionState]
/// 3. `ActionSystemFor::<A>::Map`: The mapper systems run for each registered
///    FromAction pairs with it. So this set will execute both `K => A` and
///    `G => A` mappings in this example.
///
///     - TODO: Decide if the following steps are in their own system set, or
///       just part of the mapping:
///     - The mapper system checks for conditions if it's supposed to be
///       forwarding actions
///     - ActionPhaseChanges are determined based on the previous frame's data
/// 4. `ActionSystemFor::<B>::Map`: If multiple levels are defined (A => B => C)
///    they are running one after another. Since B is mapped after A, it will
///    mapped after it too so there's something to map from.
/// 5. `ActionSystem::Mapped`: Nothing is executed here, it's for ordering
/// 6. `ActionSystemFor::<K>::Trigger` and : Events for K are triggered for entities
///    that listen for it
/// 7. `ActionSystemFor::<G>::Trigger`
/// 8. `ActionSystemFor::<A>::Trigger`
/// 9. `ActionSystemFor::<B>::Trigger`
/// 10. `ActionSystem::Triggered`: Nothing is executed here, it's for ordering
#[derive(SystemSet, Hash, Debug, Clone, Eq, PartialEq)]
pub enum ActionSystem {
	/// In this stage every ActionContext resets and all [ActionState][crate::ActionState]s
	/// are moved to the previous frames [ActionState][crate::ActionState]
	/// This is independent for each [Action][crate::Action] so this stage
	/// isn't generic.
	Reset,
	/// At this stage, all the inputs from actual input devices are collected,
	/// but no mapping occurs.
	/// If you want to programmatically fire Actions, this is where you should do it.
	/// As this stage only operates on a select few known [Action][crate::Action]s
	/// such as [KeyCode][bevy::prelude::KeyCode] and they are not supposed to
	/// have mappings between each other, this stage is not generic.
	InputSocketWrite,
	/// This empty set is used to ensure systems that has to be run after
	/// mapping are definitely happen after everything has been mapped
	Mapped,
	/// This empty set is used to let other systems know that all observers
	/// have been triggered
	Triggered,
	/// TODO: Decide if needed if an alternative event based system is added
	Dispatched,
}

#[derive(SystemSet, Hash, Debug)]
#[derive_where(Clone, Eq, PartialEq)]
pub enum ActionSystemFor<A: Action> {
	SocketReadByConnectorWriteToTerminal,
	/// This is the stage where action `A` gets mapped from all its mappings
	TerminalWriteToSocket,

	/// Notify entity observers about events
	Trigger,
	/// Not used, it's only here to capture the generic `A`
	_Phantom(PhantomData<A>),
}
