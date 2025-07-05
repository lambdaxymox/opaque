use crate::hasher::TypeProjectedHasher;
use crate::build_hasher_inner::{TypeErasedBuildHasherInner, TypeProjectedBuildHasherInner};

use core::any;
use core::fmt;
use core::marker;
use alloc_crate::boxed::Box;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

/// A type-projected hash builder.
///
/// Wrapping the hash builder like this allows us to type-erase and type-project hash builders
/// as **O(1)** time operations. When passing references to type-projected or type-erased hash
/// builders around, type-erasure and type-projection are zero-cost operations, since they have
/// identical layout.
///
/// For a given hash builder type `S`, the [`TypeProjectedBuildHasher<S>`] and [`TypeErasedHasher`] data
/// types also implement the [`BuildHasher`] trait, so we can build hashers with it just as well as
/// the underlying hash builder of type `S`. The type-projected hash builder's [`BuildHasher`]
/// implementation will build the same hasher as the underlying hash builder type does with its
/// [`BuildHasher`] implementation. The type-projected and type-erased hash builders also have an
/// additional feature where they can build type-projected versions of the underlying hasher with
/// the `build_hasher_proj` method. See the [`TypeProjectedBuildHasher::build_hasher_proj`] and
/// [`TypeErasedBuildHasher::build_hasher_proj`] methods for more detail.
///
/// # Type Erasure And Type Projection
///
/// This allows for more flexible and dynamic data handling, especially when working with
/// collections of unknown or dynamic types. Some applications of this include implementing
/// heterogeneous data structures, plugin systems, and managing foreign function interface data.
/// There are two data types that are dual to each other: [`TypeProjectedBuildHasher`] and
/// [`TypeErasedBuildHasher`].
///
/// # Tradeoffs Compared To A Non-Projected Hasher
///
/// There are some tradeoffs to gaining type-erasability and type-projectability. The projected and
/// erased hash builders have identical memory layout to ensure that type projection and type
/// erasure are both **O(1)** time operations. Thus, the underlying hash builder must be stored in
/// the equivalent of a [`Box`], which carries a small performance penalty. Moreover, the hash
/// builders must carry extra metadata about the type of the underlying hash builder through its
/// [`TypeId`]. Boxing the hash builder imposes a small performance penalty at runtime, and the
/// extra metadata makes the hash builder itself a little bigger in memory. This also puts a slight
/// restriction on what kinds of hash builders can be held inside the container: the underlying
/// hash builder must be [`any::Any`], i.e. it must have a `'static` lifetime.
///
/// # See Also
///
/// - [`TypeErasedBuildHasher`]: the type-erased counterpart to [`TypeProjectedBuildHasher`].
///
/// # Examples
///
/// Using a type-projected hash builder.
///
/// ```
/// # use opaque_hash::{TypeProjectedBuildHasher, TypeProjectedHasher};
/// # use std::any::TypeId;
/// # use std::hash::{BuildHasher, DefaultHasher, RandomState};
/// #
/// let proj_build_hasher = TypeProjectedBuildHasher::new(RandomState::new());
///
/// assert_eq!(proj_build_hasher.build_hasher_type_id(), TypeId::of::<RandomState>());
///
/// // The `BuildHasher` implementation builds an unprojected hasher.
/// let hasher: DefaultHasher = proj_build_hasher.build_hasher();
///
/// // The `build_hasher_proj` method builds a type-projected hasher.
/// let proj_hasher: TypeProjectedHasher<DefaultHasher> = proj_build_hasher.build_hasher_proj();
/// ```
#[repr(transparent)]
pub struct TypeProjectedBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    inner: TypeProjectedBuildHasherInner<S>,
}

impl<S> TypeProjectedBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Returns the [`TypeId`] of the underlying hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeProjectedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// #
    /// let proj_build_hasher = TypeProjectedBuildHasher::new(RandomState::new());
    ///
    /// assert_eq!(proj_build_hasher.build_hasher_type_id(), TypeId::of::<RandomState>());
    /// ```
    #[inline]
    pub const fn build_hasher_type_id(&self) -> any::TypeId {
        self.inner.build_hasher_type_id()
    }

    /// Returns the [`TypeId`] of the hasher returned by the underlying hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeProjectedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let proj_build_hasher = TypeProjectedBuildHasher::new(RandomState::new());
    ///
    /// assert_eq!(proj_build_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// ```
    #[inline]
    pub const fn hasher_type_id(&self) -> any::TypeId {
        self.inner.hasher_type_id()
    }
}

impl<S> TypeProjectedBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Constructs a new type-projected hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeProjectedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// #
    /// let proj_build_hasher = TypeProjectedBuildHasher::new(RandomState::new());
    ///
    /// assert_eq!(proj_build_hasher.build_hasher_type_id(), TypeId::of::<RandomState>());
    /// assert_ne!(proj_build_hasher.build_hasher_type_id(), TypeId::of::<Box<RandomState>>());
    /// ```
    #[inline]
    pub fn new(build_hasher: S) -> Self {
        let inner = TypeProjectedBuildHasherInner::new(build_hasher);

        Self { inner, }
    }

    /// Constructs a new type-projected hash builder from a boxed hash builder.
    ///
    /// The underlying type of the type-projected hash builder will be the type of the hash builder
    /// held by the box.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeProjectedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// #
    /// let proj_build_hasher = TypeProjectedBuildHasher::from_boxed_build_hasher(Box::new(RandomState::new()));
    ///
    /// assert_eq!(proj_build_hasher.build_hasher_type_id(), TypeId::of::<RandomState>());
    /// assert_ne!(proj_build_hasher.build_hasher_type_id(), TypeId::of::<Box<RandomState>>());
    #[inline]
    pub fn from_boxed_build_hasher(build_hasher: Box<S>) -> Self {
        let inner = TypeProjectedBuildHasherInner::from_boxed_build_hasher(build_hasher);

        Self { inner, }
    }

    /// Returns a reference to the underlying hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeProjectedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// #
    /// let proj_build_hasher = TypeProjectedBuildHasher::from_boxed_build_hasher(Box::new(RandomState::new()));
    ///
    /// let build_hasher: &RandomState = proj_build_hasher.get_build_hasher();
    /// ```
    pub fn get_build_hasher(&self) -> &S {
        self.inner.get_build_hasher()
    }

    /// Converts the type-projected hash builder into a boxed hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeProjectedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// #
    /// let proj_build_hasher = TypeProjectedBuildHasher::new(RandomState::new());
    /// let boxed_build_hasher: Box<RandomState> = proj_build_hasher.into_boxed_build_hasher();
    ///
    /// let new_proj_build_hasher = TypeProjectedBuildHasher::from_boxed_build_hasher(boxed_build_hasher);
    ///
    /// assert_eq!(new_proj_build_hasher.build_hasher_type_id(), TypeId::of::<RandomState>());
    /// assert_ne!(new_proj_build_hasher.build_hasher_type_id(), TypeId::of::<Box<RandomState>>());
    /// ```
    pub fn into_boxed_build_hasher(self) -> Box<S> {
        self.inner.into_boxed_build_hasher()
    }
}

impl<S> hash::BuildHasher for TypeProjectedBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    type Hasher = S::Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.inner.build_hasher()
    }
}

impl<S> TypeProjectedBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Returns a type-projected hasher.
    ///
    /// To get an unprojected hasher instead of a type-projected one, use [`build_hasher`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{TypeProjectedBuildHasher, TypeProjectedHasher};
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let proj_build_hasher = TypeProjectedBuildHasher::new(RandomState::new());
    /// let proj_hasher: TypeProjectedHasher<DefaultHasher> = proj_build_hasher.build_hasher_proj();
    /// ```
    ///
    /// [`build_hasher`]: TypeProjectedBuildHasher::build_hasher
    #[inline]
    pub fn build_hasher_proj(&self) -> TypeProjectedHasher<S::Hasher> {
        let hasher = self.inner.build_hasher();

        TypeProjectedHasher::new(hasher)
    }
}

impl<S> Clone for TypeProjectedBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<S> Default for TypeProjectedBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<S> PartialEq for TypeProjectedBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + PartialEq,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        let inner_self = self.get_build_hasher();
        let inner_other = other.get_build_hasher();

        PartialEq::eq(&inner_self, &inner_other)
    }
}

impl<S> fmt::Debug for TypeProjectedBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypeProjectedBuildHasher")
            .field("inner", self.inner.get_build_hasher())
            .finish()
    }
}

impl<S> From<S> for TypeProjectedBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn from(hasher: S) -> Self {
        Self::new(hasher)
    }
}

/// A type-erased hash builder.
///
/// Wrapping the hash builder like this allows us to type-erase and type-project hash builders
/// as **O(1)** time operations. When passing references to type-projected or type-erased hash
/// builders around, type-erasure and type-projection are zero-cost operations, since they have
/// identical layout.
///
/// For a given hash builder type `S`, the [`TypeProjectedBuildHasher<S>`] and [`TypeErasedHasher`] data
/// types also implement the [`BuildHasher`] trait, so we can build hashers with it just as well as
/// the underlying hash builder of type `S`. The type-projected hash builder's [`BuildHasher`]
/// implementation will build the same hasher as the underlying hash builder type does with its
/// [`BuildHasher`] implementation. The type-projected and type-erased hash builders also have an
/// additional feature where they can build type-projected versions of the underlying hasher with
/// the `build_hasher_proj` method. See the [`TypeProjectedBuildHasher::build_hasher_proj`] and
/// [`TypeErasedBuildHasher::build_hasher_proj`] methods for more detail.
///
/// # Type Erasure And Type Projection
///
/// This allows for more flexible and dynamic data handling, especially when working with
/// collections of unknown or dynamic types. Some applications of this include implementing
/// heterogeneous data structures, plugin systems, and managing foreign function interface data.
/// There are two data types that are dual to each other: [`TypeProjectedBuildHasher`] and
/// [`TypeErasedBuildHasher`].
///
/// # Tradeoffs Compared To A Non-Projected Hasher
///
/// There are some tradeoffs to gaining type-erasability and type-projectability. The projected and
/// erased hash builders have identical memory layout to ensure that type projection and type
/// erasure are both **O(1)** time operations. Thus, the underlying hash builder must be stored in
/// the equivalent of a [`Box`], which carries a small performance penalty. Moreover, the hash
/// builders must carry extra metadata about the type of the underlying hash builder through its
/// [`TypeId`]. Boxing the hash builder imposes a small performance penalty at runtime, and the
/// extra metadata makes the hash builder itself a little bigger in memory. This also puts a slight
/// restriction on what kinds of hash builders can be held inside the container: the underlying
/// hash builder must be [`any::Any`], i.e. it must have a `'static` lifetime.
///
/// # See Also
///
/// - [`TypeProjectedBuildHasher`]: the type-projected counterpart to [`TypeErasedBuildHasher`].
///
/// # Examples
///
/// Using a type-erased hash builder.
///
/// ```
/// # use opaque_hash::{TypeErasedBuildHasher, TypeProjectedHasher};
/// # use std::any::TypeId;
/// # use std::hash::{DefaultHasher, RandomState};
/// #
/// let opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
/// #
/// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
/// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
/// #
///
/// assert_eq!(opaque_build_hasher.build_hasher_type_id(), TypeId::of::<RandomState>());
///
/// // The `build_hasher` method builds an unprojected hasher.
/// let hasher: DefaultHasher = opaque_build_hasher.build_hasher::<RandomState>();
///
/// // The `build_hasher_proj` method builds a type-projected hasher.
/// let proj_hasher: TypeProjectedHasher<DefaultHasher> = opaque_build_hasher.build_hasher_proj::<RandomState>();
/// ```
#[repr(transparent)]
pub struct TypeErasedBuildHasher {
    inner: TypeErasedBuildHasherInner,
}

impl TypeErasedBuildHasher {
    /// Returns the [`TypeId`] of the underlying hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeErasedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
    ///
    /// assert_eq!(opaque_build_hasher.build_hasher_type_id(), TypeId::of::<RandomState>());
    /// ```
    #[inline]
    pub const fn build_hasher_type_id(&self) -> any::TypeId {
        self.inner.build_hasher_type_id()
    }

    /// Returns the [`TypeId`] of the hasher returned by the underlying hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeErasedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
    ///
    /// assert_eq!(opaque_build_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// ```
    #[inline]
    pub const fn hasher_type_id(&self) -> any::TypeId {
        self.inner.hasher_type_id()
    }
}

impl TypeErasedBuildHasher {
    /// Determines whether underlying the hash builder has the given hash builder type.
    ///
    /// Returns `true` if `self` has the hash builder allocator type. Returns `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeErasedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
    ///
    /// assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());;
    /// ```
    #[inline]
    pub fn has_build_hasher_type<S>(&self) -> bool
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
    {
        self.inner.build_hasher_type_id() == any::TypeId::of::<S>()
    }

    /// Determines whether underlying the hash builder has the given hasher type.
    ///
    /// Returns `true` if `self` returns the specified hasher type. Returns `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeErasedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
    ///
    /// assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());;
    /// ```
    #[inline]
    pub fn has_hasher_type<H>(&self) -> bool
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        self.inner.hasher_type_id() == any::TypeId::of::<H>()
    }

    /// Assert the concrete types underlying a type-erased data type.
    ///
    /// This method's main use case is ensuring the type safety of an operation before projecting
    /// into the type-projected counterpart of the type-erased hasher.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hash builder of `self` does not match the
    /// requested hash builder type `S`. This method **does not** test the hasher type produced by
    /// the underlying hash builder. This method exists only to ensure the underlying integrity of
    /// the hash builder type projections and type erasures.
    #[inline]
    #[track_caller]
    fn assert_type_safety<S>(&self)
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
    {
        #[cold]
        #[cfg_attr(feature = "nightly", optimize(size))]
        #[track_caller]
        fn type_check_failed(type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_build_hasher_type::<S>() {
            type_check_failed(self.inner.build_hasher_type_id(), any::TypeId::of::<S>());
        }
    }
}

impl TypeErasedBuildHasher {
    /// Projects the type-erased hash builder reference into a type-projected hash builder
    /// reference.
    ///
    /// # Complexity Characteristics
    ///
    /// This method runs in **O(1)** time.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hash builder of `self` does not match the
    /// requested hash builder type `S`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{TypeErasedBuildHasher, TypeProjectedBuildHasher};
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
    /// #
    /// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// let proj_build_hasher: &TypeProjectedBuildHasher<RandomState> = opaque_build_hasher.as_proj::<RandomState>();
    /// ```
    #[inline]
    #[track_caller]
    pub fn as_proj<S>(&self) -> &TypeProjectedBuildHasher<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<S>();

        unsafe { &*(self as *const TypeErasedBuildHasher as *const TypeProjectedBuildHasher<S>) }
    }

    /// Projects the mutable type-erased hash builder reference into a type-projected mutable
    /// hash builder reference.
    ///
    /// # Complexity Characteristics
    ///
    /// This method runs in **O(1)** time.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hash builder of `self` does not match the
    /// requested hash builder type `S`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{TypeErasedBuildHasher, TypeProjectedBuildHasher};
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let mut opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
    /// #
    /// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// let proj_build_hasher: &mut TypeProjectedBuildHasher<RandomState> = opaque_build_hasher.as_proj_mut::<RandomState>();
    /// ```
    #[inline]
    #[track_caller]
    pub fn as_proj_mut<S>(&mut self) -> &mut TypeProjectedBuildHasher<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<S>();

        unsafe { &mut *(self as *mut TypeErasedBuildHasher as *mut TypeProjectedBuildHasher<S>) }
    }

    /// Projects the type-erased hash builder value into a type-projected hash builder value.
    ///
    /// # Complexity Characteristics
    ///
    /// This method runs in **O(1)** time.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hash builder of `self` does not match the
    /// requested hash builder type `S`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{TypeErasedBuildHasher, TypeProjectedBuildHasher};
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
    /// #
    /// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// let proj_build_hasher: TypeProjectedBuildHasher<RandomState> = opaque_build_hasher.into_proj::<RandomState>();
    /// ```
    #[inline]
    #[track_caller]
    pub fn into_proj<S>(self) -> TypeProjectedBuildHasher<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<S>();

        TypeProjectedBuildHasher {
            inner: self.inner.into_proj(),
        }
    }

    /// Erases the type-projected hash builder value into a type-erased hash builder value.
    ///
    /// Unlike the type projection methods [`as_proj`], [`as_proj_mut`], and [`into_proj`], this
    /// method never panics.
    ///
    /// # Complexity Characteristics
    ///
    /// This method runs in **O(1)** time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{TypeErasedBuildHasher, TypeProjectedBuildHasher};
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let proj_build_hasher: TypeProjectedBuildHasher<RandomState> = TypeProjectedBuildHasher::new(RandomState::new());
    /// let opaque_build_hasher: TypeErasedBuildHasher = TypeErasedBuildHasher::from_proj(proj_build_hasher);
    /// #
    /// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// ```
    ///
    /// [`as_proj`]: TypeErasedBuildHasher::as_proj,
    /// [`as_proj_mut`]: TypeErasedBuildHasher::as_proj_mut
    /// [`into_proj`]: TypeErasedBuildHasher::into_proj
    #[inline]
    pub fn from_proj<S>(proj_self: TypeProjectedBuildHasher<S>) -> Self
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Self {
            inner: TypeErasedBuildHasherInner::from_proj(proj_self.inner),
        }
    }
}

impl TypeErasedBuildHasher {
    /// Constructs a new type-erased hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeErasedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
    /// #
    /// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    ///
    /// assert_eq!(opaque_build_hasher.build_hasher_type_id(), TypeId::of::<RandomState>());
    /// assert_ne!(opaque_build_hasher.build_hasher_type_id(), TypeId::of::<Box<RandomState>>());
    /// ```
    #[inline]
    pub fn new<S>(build_hasher: S) -> Self
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_build_hasher = TypeProjectedBuildHasher::<S>::new(build_hasher);

        Self::from_proj(proj_build_hasher)
    }

    /// Constructs a new type-erased hash builder from a boxed hash builder.
    ///
    /// The underlying type of the type-erased hash builder will be the type of the hash builder
    /// held by the box.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeErasedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::from_boxed_build_hasher::<RandomState>(Box::new(RandomState::new()));
    /// #
    /// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    ///
    /// assert_eq!(opaque_build_hasher.build_hasher_type_id(), TypeId::of::<RandomState>());
    /// assert_ne!(opaque_build_hasher.build_hasher_type_id(), TypeId::of::<Box<RandomState>>());
    /// ```
    #[inline]
    pub fn from_boxed_build_hasher<S>(build_hasher: Box<S>) -> Self
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_build_hasher = TypeProjectedBuildHasher::<S>::from_boxed_build_hasher(build_hasher);

        Self::from_proj(proj_build_hasher)
    }

    /// Returns a reference to the underlying hash builder.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hash builder of `self` does not match the
    /// requested hash builder type `S`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeErasedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::from_boxed_build_hasher(Box::new(RandomState::new()));
    /// #
    /// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    ///
    /// let build_hasher: &RandomState = opaque_build_hasher.get_build_hasher::<RandomState>();
    /// ```
    #[inline]
    #[track_caller]
    pub fn get_build_hasher<S>(&self) -> &S
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<S>();

        proj_self.get_build_hasher()
    }

    /// Converts the type-erased hash builder into a boxed hash builder.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hash builder of `self` does not match the
    /// requested hash builder type `S`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypeErasedBuildHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::new(RandomState::new());
    /// #
    /// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// let boxed_build_hasher: Box<RandomState> = opaque_build_hasher.into_boxed_build_hasher::<RandomState>();
    ///
    /// let new_opaque_build_hasher = TypeErasedBuildHasher::from_boxed_build_hasher(boxed_build_hasher);
    /// #
    /// # assert!(new_opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(new_opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    ///
    /// assert_eq!(new_opaque_build_hasher.build_hasher_type_id(), TypeId::of::<RandomState>());
    /// assert_ne!(new_opaque_build_hasher.build_hasher_type_id(), TypeId::of::<Box<RandomState>>());
    /// ```
    #[track_caller]
    pub fn into_boxed_build_hasher<S>(self) -> Box<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.into_proj::<S>();

        proj_self.into_boxed_build_hasher()
    }
}

impl TypeErasedBuildHasher {
    /// Returns an unprojected hasher.
    ///
    /// The type of the hasher returned by this method is the same as calling [`BuildHasher::build_hasher`]
    /// on the underlying hash builder held by this container.
    ///
    /// To get a type-projected hasher instead of an unprojected one, use [`build_hasher_proj`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{TypeErasedBuildHasher, TypeProjectedBuildHasher};
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
    /// #
    /// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// let hasher: DefaultHasher = opaque_build_hasher.build_hasher::<RandomState>();
    /// ```
    ///
    /// [`BuildHasher::build_hasher`]: std::hash::BuildHasher::build_hasher
    /// [`build_hasher_proj`]: TypeErasedBuildHasher::build_hasher_proj
    #[track_caller]
    pub fn build_hasher<S>(&self) -> S::Hasher
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<S>();

        <TypeProjectedBuildHasher<S> as hash::BuildHasher>::build_hasher(proj_self)
    }
}

impl TypeErasedBuildHasher {
    /// Returns a type-projected hasher.
    ///
    /// To get an unprojected hasher instead of a type-projected one, use [`build_hasher`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{TypeErasedBuildHasher, TypeProjectedBuildHasher, TypeProjectedHasher};
    /// # use std::hash::{DefaultHasher, RandomState};
    /// #
    /// let opaque_build_hasher = TypeErasedBuildHasher::new::<RandomState>(RandomState::new());
    /// #
    /// # assert!(opaque_build_hasher.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_build_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// let proj_hasher: TypeProjectedHasher<DefaultHasher> = opaque_build_hasher.build_hasher_proj::<RandomState>();
    /// ```
    ///
    /// [`build_hasher`]: TypeErasedBuildHasher::build_hasher
    #[inline]
    #[track_caller]
    pub fn build_hasher_proj<S>(&self) -> TypeProjectedHasher<S::Hasher>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<S>();

        proj_self.build_hasher_proj()
    }
}

impl fmt::Debug for TypeErasedBuildHasher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("TypeErasedBuildHasher").finish()
    }
}

mod dummy {
    use super::*;

    #[allow(dead_code)]
    pub(super) struct DummyHasher {
        _do_not_construct: marker::PhantomData<()>,
    }

    impl hash::Hasher for dummy::DummyHasher {
        #[inline]
        fn finish(&self) -> u64 {
            panic!("[`DummyHasher::finish`] should never actually be called. Its purpose is to test struct layouts.");
        }

        #[inline]
        fn write(&mut self, _bytes: &[u8]) {
            panic!("[`DummyHasher::write`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }

    #[allow(dead_code)]
    pub(super) struct DummyBuildHasher {
        _do_not_construct: marker::PhantomData<()>,
    }

    impl hash::BuildHasher for dummy::DummyBuildHasher {
        type Hasher = dummy::DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            panic!("[`DummyBuildHasher::build_hasher`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }
}

#[cfg(test)]
mod build_hasher_layout_tests {
    use super::*;
    use core::mem;
    use std::hash;

    fn run_test_type_erased_build_hasher_match_sizes<S>()
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = mem::size_of::<TypeProjectedBuildHasher<S>>();
        let result = mem::size_of::<TypeErasedBuildHasher>();

        assert_eq!(result, expected, "Type Erased and Type Projected data types size mismatch");
    }

    fn run_test_type_erased_build_hasher_match_alignments<S>()
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = mem::align_of::<TypeProjectedBuildHasher<S>>();
        let result = mem::align_of::<TypeErasedBuildHasher>();

        assert_eq!(result, expected, "Type Erased and Type Projected data types alignment mismatch");
    }

    fn run_test_type_erased_build_hasher_match_offsets<S>()
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        assert_eq!(
            mem::offset_of!(TypeProjectedBuildHasher<S>, inner),
            mem::offset_of!(TypeErasedBuildHasher, inner),
            "Type Erased and Type Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $build_hasher_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_build_hasher_layout_match_sizes() {
                    run_test_type_erased_build_hasher_match_sizes::<$build_hasher_typ>();
                }

                #[test]
                fn test_build_hasher_layout_match_alignments() {
                    run_test_type_erased_build_hasher_match_alignments::<$build_hasher_typ>();
                }

                #[test]
                fn test_build_hasher_layout_match_offsets() {
                    run_test_type_erased_build_hasher_match_offsets::<$build_hasher_typ>();
                }
            }
        };
    }

    layout_tests!(random_state, hash::RandomState);
    layout_tests!(dummy_build_hasher, dummy::DummyBuildHasher);
}

#[cfg(test)]
mod assert_send_sync {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeProjectedBuildHasher<hash::RandomState>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeProjectedBuildHasher<dummy::DummyBuildHasher>>();
    }

    /*
    #[test]
    fn test_assert_not_send_not_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeErasedBuildHasher>();
    }
    */
}
