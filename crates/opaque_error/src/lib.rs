#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

use core::fmt;
use alloc_crate::alloc;

/// The error type for `try_reserve` methods.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryReserveError {
    kind: TryReserveErrorKind,
}

impl TryReserveError {
    /// Details about the allocation that caused the error
    #[inline]
    #[must_use]
    pub fn kind(&self) -> TryReserveErrorKind {
        self.kind.clone()
    }
}

/// Details of the allocation that caused a `TryReserveError`
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TryReserveErrorKind {
    /// Error due to the computed capacity exceeding the collection's maximum
    /// (usually `isize::MAX` bytes).
    CapacityOverflow,

    /// The memory allocator returned an error
    AllocError {
        /// The layout of allocation request that failed
        layout: alloc::Layout,
    },
}

impl From<TryReserveErrorKind> for TryReserveError {
    #[inline]
    fn from(kind: TryReserveErrorKind) -> Self {
        Self { kind }
    }
}

impl From<alloc::LayoutError> for TryReserveErrorKind {
    /// Always evaluates to [`TryReserveErrorKind::CapacityOverflow`].
    #[inline]
    fn from(_: alloc::LayoutError) -> Self {
        TryReserveErrorKind::CapacityOverflow
    }
}

impl fmt::Display for TryReserveError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        formatter.write_str("memory allocation failed")?;
        let reason = match self.kind {
            TryReserveErrorKind::CapacityOverflow => " because the computed capacity exceeded the collection's maximum",
            TryReserveErrorKind::AllocError { .. } => " because the memory allocator returned an error",
        };
        formatter.write_str(reason)
    }
}

#[cfg(feature = "std")]
impl core::error::Error for TryReserveError {}
