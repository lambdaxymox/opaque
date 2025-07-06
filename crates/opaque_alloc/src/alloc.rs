use crate::alloc_inner::{TypeProjectedAllocInner, TypeErasedAllocInner};
use crate::try_project_alloc_error::{TryProjectAllocErrorKind, TryProjectAllocError};

use core::any;
use core::fmt;
use core::marker;
use core::ptr::NonNull;

use alloc_crate::boxed::Box;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

/// A type-projected memory allocator.
///
/// Wrapping the memory allocator like this allows us to type-erase and type-project allocators
/// as **O(1)** time operations. When passing references to type-projected or type-erased
/// allocators around, type-erasure and type-projection are zero-cost operations, since they have
/// identical layout.
///
/// For a given allocator type `A`, the [`TypeProjectedAlloc`] and [`TypeErasedAlloc`] data types also
/// implement the [`Allocator`] trait, so we can allocate memory with it just as well as the
/// underlying allocator of type `A`.
///
/// # Type Erasure And Type Projection
///
/// This allows for more flexible and dynamic data handling, especially when working with
/// collections of unknown or dynamic types. Some applications of this include implementing
/// heterogeneous data structures, plugin systems, and managing foreign function interface data.
/// There are two data types that are dual to each other: [`TypeProjectedAlloc`] and [`TypeErasedAlloc`].
///
/// # Tradeoffs Compared To A Non-Projected Allocator
///
/// There are some tradeoffs to gaining type-erasability and type-projectability. The projected and
/// erased allocators have identical memory layout to ensure that type projection and type erasure
/// are both **O(1)** time operations. Thus, the underlying memory allocator must be stored in the
/// equivalent of a [`Box`], which carries a small performance penalty. Moreover, the allocators
/// must carry extra metadata about the type of the underlying allocator through its [`TypeId`].
/// Boxing the allocator imposes a small performance penalty at runtime, and the extra metadata
/// makes the allocator itself a little bigger in memory, though this is very minor. This also puts
/// a slight restriction on what kinds of memory allocators can be held inside the container: the
/// underlying memory allocator must be [`any::Any`], i.e. it must have a `'static` lifetime.
///
/// # See Also
///
/// - [`TypeErasedAlloc`]: The type-erased counterpart to [`TypeProjectedAlloc`].
///
/// # Examples
///
/// Using a type-projected allocator.
///
/// ```
/// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
/// # use opaque_alloc::TypeProjectedAlloc;
/// # use std::any::TypeId;
/// #
/// # #[cfg(feature = "nightly")]
/// # use std::alloc::Global;
/// #
/// # #[cfg(not(feature = "nightly"))]
/// # use opaque_allocator_api::alloc::Global;
/// #
/// let proj_alloc = TypeProjectedAlloc::new(Global);
///
/// assert_eq!(proj_alloc.allocator_type_id(), TypeId::of::<Global>());
/// ```
///
/// [`Allocator`]: std::alloc::Allocator
#[repr(transparent)]
pub struct TypeProjectedAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: TypeProjectedAllocInner<A>,
}

impl<A> TypeProjectedAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Returns the [`TypeId`] of the underlying memory allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeProjectedAlloc;
    /// # use std::any::TypeId;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let proj_alloc = TypeProjectedAlloc::new(Global);
    ///
    /// let expected = TypeId::of::<Global>();
    /// let result = proj_alloc.allocator_type_id();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }
}

impl<A> TypeProjectedAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Constructs a new type-projected memory allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeProjectedAlloc;
    /// # use std::any::TypeId;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let proj_alloc = TypeProjectedAlloc::new(Global);
    ///
    /// assert_eq!(proj_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(proj_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
    #[inline]
    pub fn new(alloc: A) -> Self {
        let inner = TypeProjectedAllocInner::new(alloc);

        Self { inner, }
    }

    /// Constructs a new type-projected memory allocator from a boxed memory allocator.
    ///
    /// The underlying type of the type-projected memory allocator will be the type of the memory
    /// allocator held by the box.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeProjectedAlloc;
    /// # use std::any::TypeId;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let proj_alloc = TypeProjectedAlloc::from_boxed_alloc(Box::new(Global));
    ///
    /// assert_eq!(proj_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(proj_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
    #[inline]
    pub fn from_boxed_alloc(alloc: Box<A>) -> Self {
        let inner = TypeProjectedAllocInner::from_boxed_alloc(alloc);

        Self { inner, }
    }

    /// Returns a reference to the underlying memory allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeProjectedAlloc;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let proj_alloc = TypeProjectedAlloc::new(Global);
    ///
    /// let alloc: &Global = proj_alloc.allocator();
    /// ```
    #[inline]
    pub fn allocator(&self) -> &A {
        self.inner.allocator()
    }

    /// Converts the type-projected allocator into a boxed memory allocator.
    ///
    /// The resulting boxed memory allocator cannot be type-projected and type-erased again
    /// unless it is converted back via a method like [`TypeProjectedAlloc::from_boxed_alloc`].
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeProjectedAlloc;
    /// # use std::any::TypeId;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let proj_alloc = TypeProjectedAlloc::new(Global);
    /// let boxed_alloc: Box<Global> = proj_alloc.into_boxed_alloc();
    ///
    /// let new_proj_alloc = TypeProjectedAlloc::from_boxed_alloc(boxed_alloc);
    ///
    /// assert_eq!(new_proj_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(new_proj_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
    pub fn into_boxed_alloc(self) -> Box<A> {
        self.inner.into_boxed_alloc()
    }
}

unsafe impl<A> alloc::Allocator for TypeProjectedAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
        self.inner.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
        unsafe {
            self.inner.deallocate(ptr, layout);
        }
    }
}

impl<A> Clone for TypeProjectedAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<A> fmt::Debug for TypeProjectedAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypeProjectedAlloc")
            .field("inner", self.inner.allocator())
            .finish()
    }
}

impl<A> Default for TypeProjectedAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<A> From<A> for TypeProjectedAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn from(alloc: A) -> Self {
        Self::new(alloc)
    }
}

/// A type-erased memory allocator.
///
/// Wrapping the memory allocator like this allows us to type-erase and type-project allocators
/// as **O(1)** time operations. When passing references to type-projected or type-erased
/// allocators around, type-erasure and type-projection are zero-cost operations, since they have
/// identical layout.
///
/// For a given allocator type `A`, the [`TypeProjectedAlloc`] and [`TypeErasedAlloc`] data types also
/// implement the [`Allocator`] trait, so we can allocate memory with it just as well as the
/// underlying allocator of type `A`.
///
/// # Type Erasure And Type Projection
///
/// This allows for more flexible and dynamic data handling, especially when working with
/// collections of unknown or dynamic types. Some applications of this include implementing
/// heterogeneous data structures, plugin systems, and managing foreign function interface data.
/// There are two data types that are dual to each other: [`TypeProjectedAlloc`] and [`TypeErasedAlloc`].
///
/// # Tradeoffs Compared To A Non-Projected Allocator
///
/// There are some tradeoffs to gaining type-erasability and type-projectability. The projected and
/// erased allocators have identical memory layout to ensure that type projection and type erasure
/// are both **O(1)** time operations. Thus, the underlying memory allocator must be stored in the
/// equivalent of a [`Box`], which carries a small performance penalty. Moreover, the allocators
/// must carry extra metadata about the type of the underlying allocator through its [`TypeId`].
/// Boxing the allocator imposes a small performance penalty at runtime, and the extra metadata
/// makes the allocator itself a little bigger in memory, though this is very minor. This also puts
/// a slight restriction on what kinds of memory allocators can be held inside the container: the
/// underlying memory allocator must be [`any::Any`], i.e. it must have a `'static` lifetime.
///
/// # See Also
///
/// - [`TypeProjectedAlloc`]: The type-projected counterpart to [`TypeErasedAlloc`].
///
/// # Examples
///
/// Using a type-erased allocator.
///
/// ```
/// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
/// # use opaque_alloc::TypeErasedAlloc;
/// # use std::any::TypeId;
/// #
/// # #[cfg(feature = "nightly")]
/// # use std::alloc::Global;
/// #
/// # #[cfg(not(feature = "nightly"))]
/// # use opaque_allocator_api::alloc::Global;
/// #
/// let opaque_alloc = TypeErasedAlloc::new::<Global>(Global);
///
/// assert_eq!(opaque_alloc.allocator_type_id(), TypeId::of::<Global>());
/// ```
///
/// [`Allocator`]: std::alloc::Allocator
#[repr(transparent)]
pub struct TypeErasedAlloc {
    inner: TypeErasedAllocInner,
}

impl TypeErasedAlloc {
    /// Returns the [`TypeId`] of the underlying memory allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeErasedAlloc;
    /// # use std::any::TypeId;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let opaque_alloc = TypeErasedAlloc::new::<Global>(Global);
    ///
    /// let expected = TypeId::of::<Global>();
    /// let result = opaque_alloc.allocator_type_id();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }
}

impl TypeErasedAlloc {
    /// Determines whether the underlying memory allocator has the given allocator type.
    ///
    /// Returns `true` if `self` has the specified memory allocator type. Returns `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeErasedAlloc;
    /// # use std::any::{Any, TypeId};
    /// # use std::ptr::NonNull;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::{Allocator, AllocError, Layout, Global};
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::{Allocator, AllocError, Layout, Global};
    /// #
    /// trait AnyAllocator: Any + Allocator + Send + Sync {}
    ///
    /// impl<A> AnyAllocator for A where A: Any + Allocator + Send + Sync {}
    ///
    /// struct BoxedAllocator(Box<dyn AnyAllocator>);
    /// # unsafe impl Allocator for BoxedAllocator {
    /// #     fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    /// #         self.0.allocate(layout)
    /// #     }
    /// #
    /// #     unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
    /// #         self.0.deallocate(ptr, layout);
    /// #     }
    /// # }
    /// #
    ///
    /// let opaque_alloc = TypeErasedAlloc::new::<Global>(Global);
    ///
    /// assert!(opaque_alloc.has_allocator_type::<Global>());
    /// assert!(!opaque_alloc.has_allocator_type::<BoxedAllocator>());
    /// ```
    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.inner.allocator_type_id() == any::TypeId::of::<A>()
    }

    /// Assert the concrete types underlying a type-erased data type.
    ///
    /// This method's main use case is ensuring the type safety of an operation before projecting
    /// into the type-projected counterpart of the type-erased allocator.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    #[inline]
    #[track_caller]
    fn assert_type_safety<A>(&self)
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        #[cold]
        #[cfg_attr(feature = "nightly", optimize(size))]
        #[track_caller]
        fn type_check_failed(type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_allocator_type::<A>() {
            type_check_failed(self.inner.allocator_type_id(), any::TypeId::of::<A>());
        }
    }
}

impl TypeErasedAlloc {
    /// Projects the type-erased allocator reference into a type-projected allocator reference.
    ///
    /// # Complexity Characteristics
    ///
    /// This method runs in **O(1)** time.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::{TypeErasedAlloc, TypeProjectedAlloc};
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let opaque_alloc = TypeErasedAlloc::new::<Global>(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let proj_alloc: &TypeProjectedAlloc<Global> = opaque_alloc.as_proj::<Global>();
    /// ```
    #[inline]
    #[track_caller]
    pub fn as_proj<A>(&self) -> &TypeProjectedAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<A>();

        unsafe { &*(self as *const TypeErasedAlloc as *const TypeProjectedAlloc<A>) }
    }

    /// Projects the mutable type-erased allocator reference into a mutable type-projected
    /// allocator reference.
    ///
    /// # Complexity Characteristics
    ///
    /// This method runs in **O(1)** time.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::{TypeErasedAlloc, TypeProjectedAlloc};
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let mut opaque_alloc = TypeErasedAlloc::new::<Global>(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let proj_alloc: &mut TypeProjectedAlloc<Global> = opaque_alloc.as_proj_mut::<Global>();
    /// ```
    #[inline]
    #[track_caller]
    pub fn as_proj_mut<A>(&mut self) -> &mut TypeProjectedAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<A>();

        unsafe { &mut *(self as *mut TypeErasedAlloc as *mut TypeProjectedAlloc<A>) }
    }

    /// Projects the type-erased allocator value into a type-projected allocator value.
    ///
    /// # Complexity Characteristics
    ///
    /// This method runs in **O(1)** time.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::{TypeErasedAlloc, TypeProjectedAlloc};
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let opaque_alloc = TypeErasedAlloc::new::<Global>(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let proj_alloc: TypeProjectedAlloc<Global> = opaque_alloc.into_proj::<Global>();
    /// ```
    #[inline]
    #[track_caller]
    pub fn into_proj<A>(self) -> TypeProjectedAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<A>();

        TypeProjectedAlloc {
            inner: self.inner.into_proj_assuming_type::<A>(),
        }
    }

    /// Erases the type-projected allocator value into a type-erased allocator value.
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
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::{TypeErasedAlloc, TypeProjectedAlloc};
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let proj_alloc: TypeProjectedAlloc<Global> = TypeProjectedAlloc::new(Global);
    /// let opaque_alloc: TypeErasedAlloc = TypeErasedAlloc::from_proj(proj_alloc);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// ```
    ///
    /// [`as_proj`]: TypeErasedAlloc::as_proj,
    /// [`as_proj_mut`]: TypeErasedAlloc::as_proj_mut
    /// [`into_proj`]: TypeErasedAlloc::into_proj
    #[inline]
    pub fn from_proj<A>(proj_self: TypeProjectedAlloc<A>) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        Self {
            inner: TypeErasedAllocInner::from_proj(proj_self.inner),
        }
    }
}

impl TypeErasedAlloc {
    /// Projects the type-erased allocator reference into a type-projected allocator reference.
    ///
    /// # Errors
    ///
    /// This method returns an error if the [`TypeId`] of the memory allocator of `self` do not
    /// match the requested allocator type `A`.
    ///
    /// # Complexity Characteristics
    ///
    /// This method runs in **O(1)** time.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::{TypeErasedAlloc, TypeProjectedAlloc};
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let opaque_alloc = TypeErasedAlloc::new::<Global>(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let proj_alloc = opaque_alloc.try_as_proj::<Global>();
    ///
    /// assert!(proj_alloc.is_ok());
    /// ```
    #[inline]
    pub fn try_as_proj<A>(&self) -> Result<&TypeProjectedAlloc<A>, TryProjectAllocError>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        if !self.has_allocator_type::<A>() {
            return Err(TryProjectAllocError::new(
                TryProjectAllocErrorKind::Allocator,
                self.allocator_type_id(),
                any::TypeId::of::<A>()
            ));
        }

        let result = unsafe { &*(self as *const TypeErasedAlloc as *const TypeProjectedAlloc<A>) };

        Ok(result)
    }

    /// Projects the mutable type-erased allocator reference into a mutable type-projected
    /// allocator reference.
    ///
    /// # Errors
    ///
    /// This method returns an error if the [`TypeId`] of the memory allocator of `self` do not
    /// match the requested allocator type `A`.
    ///
    /// # Complexity Characteristics
    ///
    /// This method runs in **O(1)** time.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::{TypeErasedAlloc, TypeProjectedAlloc};
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let mut opaque_alloc = TypeErasedAlloc::new::<Global>(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let proj_alloc = opaque_alloc.try_as_proj_mut::<Global>();
    ///
    /// assert!(proj_alloc.is_ok());
    /// ```
    #[inline]
    pub fn try_as_proj_mut<A>(&mut self) -> Result<&mut TypeProjectedAlloc<A>, TryProjectAllocError>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        if !self.has_allocator_type::<A>() {
            return Err(TryProjectAllocError::new(
                TryProjectAllocErrorKind::Allocator,
                self.allocator_type_id(),
                any::TypeId::of::<A>()
            ));
        }

        let result = unsafe { &mut *(self as *mut TypeErasedAlloc as *mut TypeProjectedAlloc<A>) };

        Ok(result)
    }

    /// Projects the type-erased allocator value into a type-projected allocator value.
    ///
    /// # Errors
    ///
    /// This method returns an error if the [`TypeId`] of the memory allocator of `self` do not
    /// match the requested allocator type `A`.
    ///
    /// # Complexity Characteristics
    ///
    /// This method runs in **O(1)** time.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::{TypeErasedAlloc, TypeProjectedAlloc};
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let opaque_alloc = TypeErasedAlloc::new::<Global>(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let proj_alloc = opaque_alloc.try_into_proj::<Global>();
    ///
    /// assert!(proj_alloc.is_ok());
    /// ```
    #[inline]
    pub fn try_into_proj<A>(self) -> Result<TypeProjectedAlloc<A>, TryProjectAllocError>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        if !self.has_allocator_type::<A>() {
            return Err(TryProjectAllocError::new(
                TryProjectAllocErrorKind::Allocator,
                self.allocator_type_id(),
                any::TypeId::of::<A>()
            ));
        }

        let result = TypeProjectedAlloc {
            inner: self.inner.into_proj_assuming_type::<A>(),
        };

        Ok(result)
    }
}

impl TypeErasedAlloc {
    /// Constructs a new type-erased memory allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeErasedAlloc;
    /// # use std::any::TypeId;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let proj_alloc = TypeErasedAlloc::new(Global);
    ///
    /// assert_eq!(proj_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(proj_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
    #[inline]
    pub fn new<A>(alloc: A) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_alloc = TypeProjectedAlloc::<A>::new(alloc);

        Self::from_proj(proj_alloc)
    }

    /// Constructs a new type-erased memory allocator from a boxed memory allocator.
    ///
    /// The underlying type of the type-erased memory allocator will be the type of the memory
    /// allocator held by the box.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeErasedAlloc;
    /// # use std::any::TypeId;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let opaque_alloc = TypeErasedAlloc::from_boxed_alloc(Box::new(Global));
    ///
    /// assert_eq!(opaque_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(opaque_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
    #[inline]
    pub fn from_boxed_alloc<A>(alloc: Box<A>) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_alloc = TypeProjectedAlloc::<A>::from_boxed_alloc(alloc);

        Self::from_proj(proj_alloc)
    }

    /// Returns a reference to the underlying memory allocator.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeErasedAlloc;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let opaque_alloc = TypeErasedAlloc::new(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    ///
    /// let alloc: &Global = opaque_alloc.allocator::<Global>();
    /// ```
    #[inline]
    #[track_caller]
    pub fn allocator<A>(&self) -> &A
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<A>();

        proj_self.allocator()
    }

    /// Converts the type-erased allocator into a boxed memory allocator.
    ///
    /// The resulting boxed memory allocator cannot be type-projected and type-erased again
    /// unless it is converted back via a method like [`TypeProjectedAlloc::from_boxed_alloc`].
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeErasedAlloc;
    /// # use std::any::TypeId;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let opaque_alloc = TypeErasedAlloc::new(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let boxed_alloc: Box<Global> = opaque_alloc.into_boxed_alloc::<Global>();
    ///
    /// let new_opaque_alloc = TypeErasedAlloc::from_boxed_alloc(boxed_alloc);
    /// #
    /// # assert!(new_opaque_alloc.has_allocator_type::<Global>());
    /// #
    ///
    /// assert_eq!(new_opaque_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(new_opaque_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
    #[track_caller]
    pub fn into_boxed_alloc<A>(self) -> Box<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.into_proj::<A>();

        proj_self.into_boxed_alloc()
    }
}

unsafe impl alloc::Allocator for TypeErasedAlloc {
    fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
        self.inner.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
        unsafe {
            self.inner.deallocate(ptr, layout);
        }
    }
}

impl TypeErasedAlloc {
    /// Clones a type-erased memory allocator.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_alloc::TypeErasedAlloc;
    /// # use std::any::TypeId;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// let opaque_alloc = TypeErasedAlloc::new(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let cloned = opaque_alloc.clone::<Global>();
    /// ```
    #[track_caller]
    pub fn clone<A>(&self) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let proj_alloc = self.as_proj::<A>();
        let cloned_proj_alloc = proj_alloc.clone();

        Self::from_proj(cloned_proj_alloc)
    }
}

impl fmt::Debug for TypeErasedAlloc {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("TypeErasedAlloc").finish()
    }
}

mod dummy {
    use super::*;
    use core::ptr::NonNull;

    #[allow(dead_code)]
    pub(super) struct DummyAlloc {
        _do_not_construct: marker::PhantomData<()>,
    }

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, _layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            panic!("[`DummyAlloc::allocate`] should never actually be called. Its purpose is to test struct layouts.");
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: alloc::Layout) {
            panic!("[`DummyAlloc::deallocate`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }
}

#[cfg(test)]
mod alloc_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_type_erased_alloc_match_sizes<A>()
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::size_of::<TypeProjectedAlloc<A>>();
        let result = mem::size_of::<TypeErasedAlloc>();

        assert_eq!(result, expected, "Type Erased and Type Projected data types size mismatch");
    }

    fn run_test_type_erased_alloc_match_alignments<A>()
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::align_of::<TypeProjectedAlloc<A>>();
        let result = mem::align_of::<TypeErasedAlloc>();

        assert_eq!(result, expected, "Type Erased and Type Projected data types alignment mismatch");
    }

    fn run_test_type_erased_alloc_match_offsets<A>()
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        assert_eq!(
            mem::offset_of!(TypeProjectedAlloc<A>, inner),
            mem::offset_of!(TypeErasedAlloc, inner),
            "Type Erased and Type Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_type_erased_alloc_inner_layout_match_sizes() {
                    run_test_type_erased_alloc_match_sizes::<$alloc_typ>();
                }

                #[test]
                fn test_type_erased_alloc_inner_layout_match_alignments() {
                    run_test_type_erased_alloc_match_alignments::<$alloc_typ>();
                }

                #[test]
                fn test_type_erased_alloc_inner_layout_match_offsets() {
                    run_test_type_erased_alloc_match_offsets::<$alloc_typ>();
                }
            }
        };
    }

    layout_tests!(global, alloc::Global);
    layout_tests!(dummy_alloc, dummy::DummyAlloc);
}

#[cfg(test)]
mod assert_send_sync {
    use super::*;

    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeProjectedAlloc<alloc::Global>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeProjectedAlloc<dummy::DummyAlloc>>();
    }
}

/*
#[cfg(test)]
mod assert_not_send_not_sync {
    use super::*;

    #[test]
    fn test_assert_not_send_not_sync() {
        fn assert_not_send_not_sync<T: Send + Sync>() {}

        assert_not_send_not_sync::<TypeErasedAlloc>();
    }
}
*/
