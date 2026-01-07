use rx_core_common::{Observable, Operator};

use crate::operator::BufferCountOperator;

pub trait ObservablePipeExtensionBufferCount<'o>: 'o + Observable + Sized + Send + Sync {
	/// # [BufferCountOperator]
	///
	/// Buffers upstream next emissions until they reach `buffer_size`, at which
	/// point the buffer is emitted downstream. A new next emission then will
	/// start a new buffer.
	///
	/// - An incomplete buffer can be emitted upon completion!
	///
	/// ## Arguments
	///
	/// - `buffer_size`: The size of the buffer. A size of `0` is invalid and
	///   will use `1` instead.
	///
	/// > The option `start_new_buffer_every_nth`, is intentionally left out, as
	/// > that would require a `Clone` bound on the values stored, but simply
	/// > buffering them does not need it and should not require it.
	/// >
	/// > If this behavior is required, it should be implemented as a new
	/// > operator called `buffer_count_every_nth`
	#[inline]
	fn buffer_count(
		self,
		buffer_size: usize,
	) -> <BufferCountOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		BufferCountOperator::new(buffer_size).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionBufferCount<'o> for O where O: 'o + Observable + Send + Sync {}
