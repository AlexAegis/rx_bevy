use bevy::prelude::*;

pub fn send_event<E: Event + Clone>(event: E) -> impl Fn(EventWriter<E>) {
	move |mut event_writer: EventWriter<E>| {
		event_writer.write(event.clone());
	}
}
