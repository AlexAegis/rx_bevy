use bevy::prelude::*;

pub fn send_message<M: Message + Clone>(message: M) -> impl Fn(MessageWriter<M>) {
	move |mut message_writer: MessageWriter<M>| {
		message_writer.write(message.clone());
	}
}
