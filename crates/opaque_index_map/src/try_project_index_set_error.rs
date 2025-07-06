use core::any;
use core::error;
use core::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TryProjectIndexSetErrorKind {
    Value,
    BuildHasher,
    Allocator,
}

/// The error type for the [`try_as_proj`], [`try_as_proj_mut`], [`try_into_proj`] on the
/// [`TypeErasedIndexSet`] type.
///
/// [`try_as_proj`]: TypeErasedIndexSer::try_as_proj
/// [`try_as_proj_mut`]: TypeErasedIndexSet::try_as_proj_mut
/// [`try_into_proj`]: TypeErasedIndexSet::try_into_proj
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryProjectIndexSetError {
    kind: TryProjectIndexSetErrorKind,
    expected: any::TypeId,
    result: any::TypeId,
}

impl TryProjectIndexSetError {
    /// Constructs a new type projection error.
    #[inline]
    pub(crate) const fn new(kind: TryProjectIndexSetErrorKind, expected: any::TypeId, result: any::TypeId) -> Self {
        Self {
            kind,
            expected,
            result,
        }
    }

    /// Returns which data type did not match.
    #[inline]
    pub const fn kind(&self) -> TryProjectIndexSetErrorKind {
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

impl fmt::Display for TryProjectIndexSetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TryProjectIndexSetErrorKind::Value => write!(
                formatter,
                "Type projection failed for value type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
            TryProjectIndexSetErrorKind::BuildHasher => write!(
                formatter,
                "Type projection failed for hash builder type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
            TryProjectIndexSetErrorKind::Allocator => write!(
                formatter,
                "Type projection failed for allocator type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
        }
    }
}

impl error::Error for TryProjectIndexSetError {}
