mod shared_subscriber;

pub use shared_subscriber::*;

pub mod prelude {
	pub use super::shared_subscriber::*;
}
