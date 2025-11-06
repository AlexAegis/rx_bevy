/// Marks a type with a distinct conceptual category to offer granular scoping
/// for blanket implementation. (Since negative trait bounds aren't a thing)
///
/// For example, it's impossible to only implement something for all observers
/// without also having to use the same implentations for all subscribers and
/// all subjects too, since they are all also observers!
pub trait WithPrimaryCategory {
	type PrimaryCategory: PrimaryCategoryMarker;
}

pub trait PrimaryCategoryMarker: private::Seal {
	const CATEGORY: PrimaryCategory;
}

pub enum PrimaryCategory {
	Observable,
	Subject,
	Observer,
	Subscriber,
}

/// Marks the struct to be primarily considered an
/// [Observable][crate::Observable].
/// It must only be used for structs that **ONLY** implement
/// [Observable][crate::Observable] but do **NOT** implement
/// [Observer][crate::Observer] and other traits as that would make it a
/// [SubjectLike][crate::SubjectLike].
#[derive(Debug)]
pub struct PrimaryCategoryObservable;

impl private::Seal for PrimaryCategoryObservable {}

impl PrimaryCategoryMarker for PrimaryCategoryObservable {
	const CATEGORY: PrimaryCategory = PrimaryCategory::Observable;
}

/// Marks the struct to be primarily considered a
/// [SubjectLike][crate::SubjectLike].
/// It must only be used for things that implement **both**
/// [Observable][crate::Observable] and [Observer][crate::Observer]!
#[derive(Debug)]
pub struct PrimaryCategorySubject;

impl private::Seal for PrimaryCategorySubject {}

impl PrimaryCategoryMarker for PrimaryCategorySubject {
	const CATEGORY: PrimaryCategory = PrimaryCategory::Subject;
}

/// Marks the struct to be primarily considered an [Observer][crate::Observer].
/// It must only be used for structs that **ONLY** implement
/// [Observer][crate::Observer] but do **NOT** implement
/// [SubscriptionLike][crate::SubscriptionLike] as that would make it a
/// [Subscriber][crate::Subscriber].
#[derive(Debug)]
pub struct PrimaryCategoryObserver;

impl private::Seal for PrimaryCategoryObserver {}

impl PrimaryCategoryMarker for PrimaryCategoryObserver {
	const CATEGORY: PrimaryCategory = PrimaryCategory::Observer;
}

/// Marks the struct to be primarily considered a
/// [Subscriber][crate::Subscriber].
/// It must only be used for things that implement **both**
/// [Observer][crate::Observer] and [SubscriptionLike][crate::SubscriptionLike]!
#[derive(Debug)]
pub struct PrimaryCategorySubscriber;

impl private::Seal for PrimaryCategorySubscriber {}

impl PrimaryCategoryMarker for PrimaryCategorySubscriber {
	const CATEGORY: PrimaryCategory = PrimaryCategory::Subscriber;
}

/// 🦭
mod private {
	pub trait Seal {}
}
