use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_egui::{EguiContextPass, EguiContexts, egui::Id};
use derive_where::derive_where;

use crate::{Action, ActionSocket, EntityAndName};

#[derive_where(Default)]
pub struct ActionSignalDebugUiPlugin<A: Action> {
	_phantom_data_action: PhantomData<A>,
}

impl<A: Action> Plugin for ActionSignalDebugUiPlugin<A> {
	fn build(&self, app: &mut App) {
		app.add_systems(EguiContextPass, draw_socket_data::<A>);
	}
}

fn draw_socket_data<A: Action>(
	mut egui: EguiContexts,
	query: Query<(Entity, &ActionSocket<A>, Option<&Name>)>,
) {
	bevy_egui::egui::Window::new(format!(
		"ActionSocket<{}> Debug Window",
		std::any::type_name::<A>()
	))
	.id(Id::new(format!(
		"egui window {}",
		std::any::type_name::<A>()
	)))
	.show(egui.ctx_mut(), |ui| {
		for (entity, socket, name) in query.iter() {
			ui.push_id(entity, |ui| {
				ui.label(format!(
					"Socket (Entity: {})",
					Into::<EntityAndName>::into((entity, name))
				));
				for (action, signal) in socket.iter_signals() {
					ui.label(format!("\t{:?}:\t\t{:?}", action, signal));
				}
			});
		}
	});
}
