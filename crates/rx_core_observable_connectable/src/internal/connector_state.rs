use std::sync::{Arc, Mutex};

use rx_core_common::{
	LockWithPoisonBehavior, Observable, ObservableOutput, Provider, SubjectLike, TeardownCollection,
};
use rx_core_macro_observable_derive::RxObservable;

use crate::{
	internal::{Connection, ConnectionState, ConnectionSubscriber},
	observable::{ConnectableOptions, ConnectionHandle, ConnectionSubscription},
};

#[derive(RxObservable)]
#[rx_out(<ConnectorProvider::Provided as ObservableOutput>::Out)]
#[rx_out_error(<ConnectorProvider::Provided as ObservableOutput>::OutError)]
pub(crate) struct ConnectorState<Source, ConnectorProvider>
where
	Source: Observable,
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike<In = Source::Out, InError = Source::OutError> + Clone,
	ConnectionSubscription<Source, ConnectorProvider::Provided>: 'static + TeardownCollection,
{
	/// Upon connection, the connector subject will subscribe to this source
	/// observable
	source: Source,

	/// Upon subscription this connector subject is what will be used as the
	/// source
	connector: Arc<Mutex<Option<ConnectorProvider::Provided>>>,

	connection: Arc<Mutex<Connection<ConnectionSubscription<Source, ConnectorProvider::Provided>>>>,

	connection_state: Arc<Mutex<ConnectionState>>,

	options: ConnectableOptions<ConnectorProvider>,
}

impl<Source, ConnectorProvider> ConnectorState<Source, ConnectorProvider>
where
	Source: Observable,
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike<In = Source::Out, InError = Source::OutError> + Clone,
	ConnectionSubscription<Source, ConnectorProvider::Provided>: 'static + TeardownCollection,
{
	pub(crate) fn new(
		source: Source,
		connection: Arc<
			Mutex<Connection<ConnectionSubscription<Source, ConnectorProvider::Provided>>>,
		>,
		connection_state: Arc<Mutex<ConnectionState>>,
		options: ConnectableOptions<ConnectorProvider>,
	) -> Self {
		Self {
			source,
			connector: Arc::new(Mutex::new(None)),
			connection,
			connection_state,
			options,
		}
	}

	pub(crate) fn get_connector(&mut self) -> ConnectorProvider::Provided {
		let mut connector = self.connector.lock_ignore_poison();
		connector
			.get_or_insert_with(|| self.options.connector_provider.provide())
			.clone()
	}

	pub(crate) fn create_connection(
		&mut self,
	) -> ConnectionSubscription<Source, ConnectorProvider::Provided> {
		let connector = self.get_connector().clone();

		let reset_connector_on_disconnect = self.options.reset_connector_on_disconnect;
		let connection_on_complete = self.connection_state.clone();
		let connection_on_error = self.connection_state.clone();
		let connector_on_complete = self.connector.clone();
		let connector_on_error = self.connector.clone();
		let connector_on_unsubscribe = self.connector.clone();
		let reset_on_complete = self.options.reset_connector_on_complete;
		let reset_on_error = self.options.reset_connector_on_error;

		self.source.subscribe(ConnectionSubscriber::new(
			connector.clone(),
			Box::new(move || {
				let mut connection_state = connection_on_complete.lock_ignore_poison();

				if reset_on_complete {
					connector_on_complete.lock_ignore_poison().take();
					connection_state.reset();
				}
				connection_state.completed();
			}),
			Box::new(move || {
				let mut connection_state = connection_on_error.lock_ignore_poison();
				if reset_on_error {
					connector_on_error.lock_ignore_poison().take();
					connection_state.reset();
				}
				connection_state.errored();
			}),
			Box::new(move || {
				if reset_connector_on_disconnect {
					// Simply drop the connector from behind the mutex, so that
					// new connections will be forced to create a new connector.
					connector_on_unsubscribe.lock_ignore_poison().take();
					// Attempting to unsubscribe the connector here will lead to a deadlock!
				}
			}),
		))
	}

	pub(crate) fn register_connection(
		&mut self,
		connection: ConnectionSubscription<Source, ConnectorProvider::Provided>,
	) -> ConnectionHandle<ConnectionSubscription<Source, ConnectorProvider::Provided>> {
		self.connection
			.lock_clear_poison()
			.register_connection(connection)
	}

	pub(crate) fn connect(
		&mut self,
	) -> ConnectionHandle<ConnectionSubscription<Source, ConnectorProvider::Provided>> {
		let active_connection = self.connection.lock_ignore_poison().get_active_connection();

		active_connection.unwrap_or_else(move || {
			let new_connection = self.create_connection();
			self.register_connection(new_connection)
		})
	}

	#[inline]
	pub(crate) fn disconnect(&mut self) -> bool {
		self.connection.lock_ignore_poison().disconnect()
	}

	pub(crate) fn reset(&mut self) {
		self.disconnect();
		self.connector.lock_ignore_poison().take();
	}
}
