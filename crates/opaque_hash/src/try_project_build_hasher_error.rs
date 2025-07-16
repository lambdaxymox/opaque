use core::any;
use core::error;
use core::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TryProjectBuildHasherErrorKind {
    BuildHasher,
}

/// The error type for the [`try_as_proj`], [`try_as_proj_mut`], [`try_into_proj`] on the
/// [`TypeErasedBuildHasher`] type.
///
/// [`try_as_proj`]: TypeErasedBuildHasher::try_as_proj
/// [`try_as_proj_mut`]: TypeErasedBuildHasher::try_as_proj_mut
/// [`try_into_proj`]: TypeErasedBuildHasher::try_into_proj
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryProjectBuildHasherError {
    kind: TryProjectBuildHasherErrorKind,
    expected: any::TypeId,
    result: any::TypeId,
}

impl TryProjectBuildHasherError {
    /// Constructs a new type projection error.
    #[inline]
    pub(crate) const fn new(kind: TryProjectBuildHasherErrorKind, expected: any::TypeId, result: any::TypeId) -> Self {
        Self { kind, expected, result }
    }

    /// Returns which data type did not match.
    #[inline]
    pub const fn kind(&self) -> TryProjectBuildHasherErrorKind {
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

impl fmt::Display for TryProjectBuildHasherError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TryProjectBuildHasherErrorKind::BuildHasher => write!(
                formatter,
                "Type projection failed for hash builder type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
        }
    }
}

impl error::Error for TryProjectBuildHasherError {}
