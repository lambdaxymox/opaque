use core::any;
use core::error;
use core::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TryProjectIndexMapErrorKind {
    Key,
    Value,
    BuildHasher,
    Allocator,
}

/// The error type for the [`try_as_proj`], [`try_as_proj_mut`], [`try_into_proj`] on the
/// [`TypeErasedIndexMap`] type.
///
/// [`try_as_proj`]: TypeErasedIndexMap::try_as_proj
/// [`try_as_proj_mut`]: TypeErasedIndexMap::try_as_proj_mut
/// [`try_into_proj`]: TypeErasedIndexMap::try_into_proj
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryProjectIndexMapError {
    kind: TryProjectIndexMapErrorKind,
    expected: any::TypeId,
    result: any::TypeId,
}

impl TryProjectIndexMapError {
    /// Constructs a new type projection error.
    #[inline]
    pub(crate) const fn new(kind: TryProjectIndexMapErrorKind, expected: any::TypeId, result: any::TypeId) -> Self {
        Self {
            kind,
            expected,
            result,
        }
    }

    /// Returns which data type did not match.
    #[inline]
    pub const fn kind(&self) -> TryProjectIndexMapErrorKind {
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

impl fmt::Display for TryProjectIndexMapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TryProjectIndexMapErrorKind::Key => write!(
                formatter,
                "Type projection failed for key type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
            TryProjectIndexMapErrorKind::Value => write!(
                formatter,
                "Type projection failed for value type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
            TryProjectIndexMapErrorKind::BuildHasher => write!(
                formatter,
                "Type projection failed for hash builder type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
            TryProjectIndexMapErrorKind::Allocator => write!(
                formatter,
                "Type projection failed for allocator type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
        }
    }
}

impl error::Error for TryProjectIndexMapError {}
