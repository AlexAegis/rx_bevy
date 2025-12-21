use derive_where::derive_where;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_observable_connectable::observable::{
	Connectable, ConnectableObservable, ConnectableOptions, ConnectionHandle,
};
use rx_core_subject_publish::subject::PublishSubject;
use rx_core_traits::prelude::*;

use crate::operator::ShareOptions;

#[derive_where(Clone)]
#[derive(RxObservable)]
#[rx_out(Source::Out)]
#[rx_out_error(Source::OutError)]
pub struct ShareObservable<Source>
where
	Source: Observable,
	Source::Out: Clone,
	Source::OutError: Clone,
{
	connectable: ConnectableObservable<
		Source,
		PublishSubject<Source::Out, Source::OutError>,
	>,
	connection: Option<
		ConnectionHandle<
			<ConnectableObservable<
				Source,
				PublishSubject<Source::Out, Source::OutError>,
			> as Connectable>::ConnectionSubscription,
		>,
	>,
}

impl<Source> ShareObservable<Source>
where
	Source: Observable,
	Source::Out: Clone,
	Source::OutError: Clone,
{
	pub fn new(source: Source, options: ShareOptions<Source::Out, Source::OutError>) -> Self {
		Self {
			connectable: ConnectableObservable::new(
				source,
				ConnectableOptions::new(options.connector_creator),
			),
			connection: None,
		}
	}

	pub fn is_connected(&self) -> bool {
		self.connection
			.as_ref()
			.map(|connection| !connection.is_closed())
			.unwrap_or(false)
	}

	pub fn connect(
		&mut self,
	) -> ConnectionHandle<
		<ConnectableObservable<
			Source,
			PublishSubject<Source::Out, Source::OutError>,
		> as Connectable>::ConnectionSubscription,
	>{
		if let Some(connection) = self.connection.as_ref()
			&& !connection.is_closed()
		{
			return connection.clone();
		}

		let connection = self.connectable.connect();
		self.connection = Some(connection.clone());
		connection
	}

	pub fn disconnect(&mut self) -> bool {
		if let Some(mut connection) = self.connection.take()
			&& !connection.is_closed()
		{
			connection.unsubscribe();
			return true;
		}

		false
	}
}

impl<Source> Observable for ShareObservable<Source>
where
	Source: Observable,
	Source::Out: Clone,
	Source::OutError: Clone,
{
	type Subscription<Destination>
		= <ConnectableObservable<
		Source,
		PublishSubject<Source::Out, Source::OutError>,
	> as Observable>::Subscription<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ rx_core_traits::UpgradeableObserver<In = Self::Out, InError = Self::OutError>
			+ Send
			+ Sync,
	{
		let subscription = self.connectable.subscribe(destination);

		self.connect();

		subscription
	}
}
