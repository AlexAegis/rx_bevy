mod mock_clock;
mod mock_context;
mod mock_observer;

pub use mock_clock::*;
pub use mock_context::*;
pub use mock_observer::*;

pub mod prelude {
	pub use super::mock_clock::*;
	pub use super::mock_context::*;
	pub use super::mock_observer::*;
}
