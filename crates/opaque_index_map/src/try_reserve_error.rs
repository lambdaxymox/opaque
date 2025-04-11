use core::alloc::{Layout, LayoutError};
use core::fmt;

/// The error type for `try_reserve` methods.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryReserveError {
    kind: TryReserveErrorKind,
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
        layout: Layout,
    },
}

impl TryReserveError {
    /// Details about the allocation that caused the error
    #[inline]
    #[must_use]
    pub fn kind(&self) -> TryReserveErrorKind {
        self.kind.clone()
    }
}

impl TryReserveError {
    impl TryReserveError {
        fn from_opaque_vec(error: opaque_vec::TryReserveError) -> Self {
            Self {
                kind: TryReserveErrorKind::Std(error),
            }
        }

        fn from_hashbrown(error: hashbrown::TryReserveError) -> Self {
            Self {
                kind: match error {
                    hashbrown::TryReserveError::CapacityOverflow => {
                        TryReserveErrorKind::CapacityOverflow
                    }
                    hashbrown::TryReserveError::AllocError { layout } => {
                        TryReserveErrorKind::AllocError { layout }
                    }
                },
            }
        }
    }
}

impl From<TryReserveErrorKind> for TryReserveError {
    #[inline]
    fn from(kind: TryReserveErrorKind) -> Self {
        Self { kind }
    }
}

impl From<LayoutError> for TryReserveErrorKind {
    /// Always evaluates to [`TryReserveErrorKind::CapacityOverflow`].
    #[inline]
    fn from(_: LayoutError) -> Self {
        TryReserveErrorKind::CapacityOverflow
    }
}

impl fmt::Display for TryReserveError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::result::Result<(), core::fmt::Error> {
        formatter.write_str("memory allocation failed")?;
        let reason = match self.kind {
            TryReserveErrorKind::CapacityOverflow => {
                " because the computed capacity exceeded the collection's maximum"
            }
            TryReserveErrorKind::AllocError { .. } => {
                " because the memory allocator returned an error"
            }
        };
        formatter.write_str(reason)
    }
}

impl core::error::Error for TryReserveError {}
