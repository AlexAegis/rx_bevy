use bevy::prelude::*;

pub fn send_message<M: Event + Clone>(message: M) -> impl Fn(EventWriter<M>) {
	move |mut message_writer: EventWriter<M>| {
		message_writer.write(message.clone());
	}
}
