use rx_core_common::{WorkContext, WorkContextProvider};

/// A [`WorkContextProvider`] that provides no context.
///
/// For use in environments where no external state needs to be
/// passed to scheduled work, like standalone CLI applications.
pub struct UnitContext;

impl WorkContextProvider for UnitContext {
	type Item<'c> = UnitContextItem;
}

/// The context item for [`UnitContext`]. Carries no data.
pub struct UnitContextItem;

impl WorkContext<'_> for UnitContextItem {}
