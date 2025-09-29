mod switch_subscriber;
mod switch_subscriber_state;

pub use switch_subscriber::*;
pub use switch_subscriber_state::*;

pub mod prelude {
	pub use super::switch_subscriber::*;
}
