use crate::observable::ClosedObservable;

/// # [ClosedObservable]
///
/// An observable that immediately closes without completing or emitting any
/// values.
///
/// ## See also:
///
/// - [`empty`]: Completes immediately without emitting any values.
/// - [`never`]: Never emits anything, never closes!
pub fn closed() -> ClosedObservable {
	ClosedObservable
}
