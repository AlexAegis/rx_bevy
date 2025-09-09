use bevy_ecs::system::Commands;
use rx_bevy_core::ObserverInput;

pub trait Observer: ObserverInput + SignalContext {
	fn next(&mut self, next: Self::In, context: &mut Self::Context);
	fn error(&mut self, error: Self::InError, context: &mut Self::Context);
	fn complete(&mut self, context: &mut Self::Context);
}

pub trait SubscriptionLike: SignalContext {
	fn unsubscribe(&mut self, context: &mut Self::Context);

	fn is_closed(&self) -> bool;
}

pub trait SignalContext {
	type Context;
}

pub struct CommandContext {
	commands: Commands<'static, 'static>,
}

impl CommandContext {
	pub fn new<'w, 's>(commands: Commands<'w, 's>) -> Self {
		// SAFETY: asd
		let commands: Commands<'static, 'static> = unsafe {
			std::mem::transmute::<Commands<'w, 's>, Commands<'static, 'static>>(commands)
		};
		Self { commands }
	}
}

fn goo(mut commands: Commands, ctx: CommandContext) {
	let c = CommandContext::new(commands);
}
