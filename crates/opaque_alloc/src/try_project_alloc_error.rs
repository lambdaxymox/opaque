use core::any;
use core::error;
use core::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TryProjectAllocErrorKind {
    Allocator,
}

/// The error type for the [`try_as_proj`], [`try_as_proj_mut`], [`try_into_proj`] on the
/// [`TypeErasedAlloc`] type.
///
/// [`try_as_proj`]: TypeErasedAlloc::try_as_proj
/// [`try_as_proj_mut`]: TypeErasedAlloc::try_as_proj_mut
/// [`try_into_proj`]: TypeErasedAlloc::try_into_proj
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryProjectAllocError {
    kind: TryProjectAllocErrorKind,
    expected: any::TypeId,
    result: any::TypeId,
}

impl TryProjectAllocError {
    /// Constructs a new type projection error.
    #[inline]
    pub(crate) const fn new(kind: TryProjectAllocErrorKind, expected: any::TypeId, result: any::TypeId) -> Self {
        Self {
            kind,
            expected,
            result,
        }
    }

    /// Returns which data type did not match.
    #[inline]
    pub const fn kind(&self) -> TryProjectAllocErrorKind {
        self.kind
    }

    /// Returns the [`TypeId`] of the expected type to perform the type projection.
    #[inline]
    pub const fn expected(&self) -> any::TypeId {
        self.expected
    }

    /// Returns the [`TypeId`] of the provided type to perform the type projection.
    #[inline]
    pub const fn result(&self) -> any::TypeId {
        self.result
    }
}

impl fmt::Display for TryProjectAllocError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TryProjectAllocErrorKind::Allocator => write!(
                formatter,
                "Type projection failed for allocator type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
        }
    }
}

impl error::Error for TryProjectAllocError {}
