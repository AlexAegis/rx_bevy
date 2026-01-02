use rx_core_macro_subject_derive::RxSubject;
use rx_core_operator_filter::operator::FilterOperator;
use rx_core_operator_map::operator::MapOperator;
use rx_core_subject_behavior::subject::BehaviorSubject;
use rx_core_subject_publish::internal::MulticastSubscription;
use rx_core_traits::{Never, Observable, Observer, Pipe, Signal, Subscriber, UpgradeableObserver};

/// # [ProvenanceSubject]
///
/// A ProvenanceSubject is a BehaviorSubject that keeps track of where updated
/// had come from. It provides filtered and unfiltered observables
/// to observe the state.
///
/// It's useful when you want to selectively react to updates based on who
/// issued them.
///
/// Subscribing to this subject directly will emit the provenance data too.
/// While that can be useful, the recommended way to subscribe to this subject
/// is by using the pre-made filtered and mapped outputs that strips off the
/// provenance data for you:
///
/// - all
/// - initial_then_by_provenance
/// - only_by_provenance
///
#[derive(RxSubject, Clone)]
#[rx_in((In, Provenance))]
#[rx_in_error(InError)]
#[rx_out((In, Provenance))]
#[rx_out_error(InError)]
#[rx_delegate_subscription_like_to_destination]
pub struct ProvenanceSubject<Provenance, In, InError = Never>
where
	Provenance: Signal + Clone + PartialEq,
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[destination]
	subject: BehaviorSubject<(In, Provenance), InError>,
}

impl<Provenance, In, InError> Default for ProvenanceSubject<Provenance, In, InError>
where
	Provenance: Signal + Clone + PartialEq + Default,
	In: Signal + Clone + Default,
	InError: Signal + Clone,
{
	fn default() -> Self {
		Self::new(In::default(), Provenance::default())
	}
}

impl<Provenance, In, InError> ProvenanceSubject<Provenance, In, InError>
where
	Provenance: Signal + Clone + PartialEq,
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub fn new(value: In, provenance: Provenance) -> Self {
		Self {
			subject: BehaviorSubject::new((value, provenance)),
		}
	}

	/// Returns a clone of the currently stored value
	/// In case you want to access the current value, prefer using a
	/// subscription to keep your code reactive; only use this when it's
	/// absolutely necessary.
	#[inline]
	pub fn value(&self) -> (In, Provenance) {
		self.subject.value()
	}

	pub fn all(
		&self,
	) -> Pipe<
		BehaviorSubject<(In, Provenance), InError>,
		MapOperator<(In, Provenance), InError, fn((In, Provenance)) -> In, In>,
	> {
		Pipe::new(
			self.subject.clone(),
			MapOperator::new(strip_provenance::<In, Provenance>),
		)
	}

	/// Always replays the stored value regardless of provenance, but then
	/// only emit values of matching provenance.
	pub fn initial_then_by_provenance(
		&self,
		by_provenance: Provenance,
	) -> Pipe<
		Pipe<
			BehaviorSubject<(In, Provenance), InError>,
			FilterOperator<
				(In, Provenance),
				InError,
				impl Fn(&(In, Provenance), usize) -> bool
				+ Clone
				+ Send
				+ Sync
				+ use<Provenance, In, InError>,
			>,
		>,
		MapOperator<(In, Provenance), InError, fn((In, Provenance)) -> In, In>,
	> {
		Pipe::new(
			Pipe::new(
				self.subject.clone(),
				FilterOperator::new(create_provenance_filter_always_replay::<In, Provenance>(
					by_provenance,
				)),
			),
			MapOperator::new(strip_provenance::<In, Provenance>),
		)
	}

	/// Only emit values of matching provenance. If the initial value is not of
	/// matching provenance, **no** replay will happen! If you need both
	/// filtering and an initial value, use `initial_then_by_provenance`.
	pub fn only_by_provenance(
		&self,
		by_provenance: Provenance,
	) -> Pipe<
		Pipe<
			BehaviorSubject<(In, Provenance), InError>,
			FilterOperator<
				(In, Provenance),
				InError,
				impl Fn(&(In, Provenance), usize) -> bool
				+ Clone
				+ Send
				+ Sync
				+ use<Provenance, In, InError>,
			>,
		>,
		MapOperator<(In, Provenance), InError, fn((In, Provenance)) -> In, In>,
	> {
		Pipe::new(
			Pipe::new(
				self.subject.clone(),
				FilterOperator::new(create_provenance_filter::<In, Provenance>(by_provenance)),
			),
			MapOperator::new(strip_provenance::<In, Provenance>),
		)
	}
}

impl<Provenance, In, InError> Observer for ProvenanceSubject<Provenance, In, InError>
where
	Provenance: Signal + Clone + PartialEq,
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: (In, Provenance)) {
		self.subject.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.subject.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.subject.complete();
	}
}

impl<Provenance, In, InError> Observable for ProvenanceSubject<Provenance, In, InError>
where
	Provenance: Signal + Clone + PartialEq,
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type Subscription<Destination>
		= MulticastSubscription<(In, Provenance), InError>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	#[inline]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		self.subject.subscribe(destination.upgrade())
	}
}

fn strip_provenance<In, Provenance>((value, _provenance): (In, Provenance)) -> In {
	value
}

fn create_provenance_filter<In, Provenance>(
	for_provenance: Provenance,
) -> impl Fn(&(In, Provenance), usize) -> bool + Clone
where
	Provenance: PartialEq + Clone,
{
	move |(_value, provenance), _index| provenance == &for_provenance
}

fn create_provenance_filter_always_replay<In, Provenance>(
	for_provenance: Provenance,
) -> impl Fn(&(In, Provenance), usize) -> bool + Clone
where
	Provenance: PartialEq + Clone,
{
	move |(_value, provenance), index| index == 0 || provenance == &for_provenance
}
