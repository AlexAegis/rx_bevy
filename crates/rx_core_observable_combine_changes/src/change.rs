#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Change<T> {
	JustUpdated(T),
	Latest(T),
	None,
}
