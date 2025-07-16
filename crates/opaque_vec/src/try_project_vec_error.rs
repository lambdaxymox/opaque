use core::any;
use core::error;
use core::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TryProjectVecErrorKind {
    Element,
    Allocator,
}

/// The error type for the [`try_as_proj`], [`try_as_proj_mut`], [`try_into_proj`] on the
/// [`TypeErasedVec`] type.
///
/// [`try_as_proj`]: TypeErasedVec::try_as_proj
/// [`try_as_proj_mut`]: TypeErasedVec::try_as_proj_mut
/// [`try_into_proj`]: TypeErasedVec::try_into_proj
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryProjectVecError {
    kind: TryProjectVecErrorKind,
    expected: any::TypeId,
    result: any::TypeId,
}

impl TryProjectVecError {
    /// Constructs a new type projection error.
    #[inline]
    pub(crate) const fn new(kind: TryProjectVecErrorKind, expected: any::TypeId, result: any::TypeId) -> Self {
        Self { kind, expected, result }
    }

    /// Returns which data type did not match.
    #[inline]
    pub const fn kind(&self) -> TryProjectVecErrorKind {
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

impl fmt::Display for TryProjectVecError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TryProjectVecErrorKind::Element => write!(
                formatter,
                "Type projection failed for element type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
            TryProjectVecErrorKind::Allocator => write!(
                formatter,
                "Type projection failed for allocator type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
        }
    }
}

impl error::Error for TryProjectVecError {}
