use core::any;
use core::marker;
use core::mem;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ptr;
use core::ptr::NonNull;
use alloc_crate::alloc;
use alloc_crate::boxed::Box;

use opaque_range_types::UsizeNoHighBit;
use opaque_alloc::{OpaqueAlloc, TypedProjAlloc};
use opaque_error::{TryReserveError, TryReserveErrorKind};

// One central function responsible for reporting capacity overflows. This will
// ensure that the code generation related to these panics is minimal as there's
// only one location which panics rather than a bunch throughout the module.
#[inline(never)]
#[track_caller]
fn capacity_overflow() -> ! {
    panic!("capacity overflow");
}

enum AllocInit {
    /// The contents of the new memory are uninitialized.
    Uninitialized,
    /// The new memory is guaranteed to be zeroed.
    Zeroed,
}

type Capacity = UsizeNoHighBit;

const ZERO_CAPACITY: Capacity = unsafe { Capacity::new_unchecked(0) };

pub(crate) unsafe fn new_capacity<T>(capacity: usize) -> Capacity {
    if crate::zst::is_zst::<T>() {
        ZERO_CAPACITY
    } else {
        unsafe { Capacity::new_unchecked(capacity) }
    }
}

const fn min_non_zero_capacity(size: usize) -> usize {
    if size == 1 {
        8
    } else if size <= 1024 {
        4
    } else {
        1
    }
}

// Central function for reserve error handling.
#[cold]
#[optimize(size)]
#[track_caller]
fn handle_error(err: TryReserveError) -> ! {
    match err.kind() {
        TryReserveErrorKind::CapacityOverflow => capacity_overflow(),
        TryReserveErrorKind::AllocError { layout, .. } => alloc::handle_alloc_error(layout),
    }
}

#[inline]
fn layout_array(capacity: usize, element_layout: alloc::Layout) -> Result<alloc::Layout, TryReserveError> {
    element_layout
        .repeat(capacity)
        .map(|(layout, _pad)| layout)
        .map_err(|_| TryReserveError::from(TryReserveErrorKind::CapacityOverflow))
}

// We need to guarantee the following:
// * We don't ever allocate `> isize::MAX` byte-size objects.
// * We don't overflow `usize::MAX` and actually allocate too little.
//
// On 64-bit we just need to check for overflow since trying to allocate
// `> isize::MAX` bytes will surely fail. On 32-bit and 16-bit we need to add
// an extra guard for this in case we're running on a platform which can use
// all 4GB in user-space, e.g., PAE or x32.
#[inline]
fn alloc_guard(alloc_size: usize) -> Result<(), TryReserveError> {
    if usize::BITS < 64 && alloc_size > isize::MAX as usize {
        Err(TryReserveError::from(
            TryReserveErrorKind::CapacityOverflow,
        ))
    } else {
        Ok(())
    }
}

// not marked inline(never) since we want optimizers to be able to observe the specifics of this
// function, see tests/codegen/vec-reserve-extend.rs.
#[cold]
fn finish_grow<A>(
    new_layout: alloc::Layout,
    current_memory: Option<(NonNull<u8>, alloc::Layout)>,
    alloc: &mut A,
) -> Result<NonNull<[u8]>, TryReserveError>
where
    A: alloc::Allocator,
{
    alloc_guard(new_layout.size())?;

    let memory = if let Some((ptr, old_layout)) = current_memory {
        debug_assert_eq!(old_layout.align(), new_layout.align());
        unsafe {
            // The allocator checks for alignment equality
            std::hint::assert_unchecked(old_layout.align() == new_layout.align());
            alloc.grow(ptr, old_layout, new_layout)
        }
    } else {
        alloc.allocate(new_layout)
    };

    memory.map_err(|_| TryReserveError::from(TryReserveErrorKind::AllocError { layout: new_layout }))
}

#[repr(transparent)]
pub(crate) struct TypedProjRawVec<T, A = alloc::Global>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: RawVecMemory,
    _marker: marker::PhantomData<(T, A)>,
}

unsafe impl<T, A> Send for TypedProjRawVec<T, A>
where
    T: any::Any + Send,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

unsafe impl<T, A> Sync for TypedProjRawVec<T, A>
where
    T: any::Any + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, A> TypedProjRawVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) const fn capacity(&self) -> usize {
        self.inner.capacity(size_of::<T>())
    }
}

impl<T, A> TypedProjRawVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) const fn ptr(&self) -> *mut T {
        self.inner.ptr()
    }

    #[inline]
    pub(crate) const fn non_null(&self) -> NonNull<T> {
        self.inner.non_null()
    }

    #[inline]
    pub(crate) fn allocator(&self) -> &TypedProjAlloc<A> {
        self.inner.allocator()
    }
}

impl<T, A> TypedProjRawVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn new_in(alloc: TypedProjAlloc<A>) -> Self {
        let element_layout = alloc::Layout::new::<T>();

        Self {
            inner: RawVecMemory::new_in(alloc, element_layout),
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn with_capacity_in(capacity: usize, alloc: TypedProjAlloc<A>) -> Self {
        let element_layout = alloc::Layout::new::<T>();

        Self {
            inner: RawVecMemory::with_capacity_in(capacity, alloc, element_layout),
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    pub(crate) fn try_with_capacity_in(capacity: usize, alloc: TypedProjAlloc<A>) -> Result<Self, TryReserveError> {
        let element_layout = alloc::Layout::new::<T>();

        match RawVecMemory::try_with_capacity_in(capacity, alloc, element_layout) {
            Ok(inner) => Ok(Self { inner, _marker: marker::PhantomData }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn with_capacity_zeroed_in(capacity: usize, alloc: TypedProjAlloc<A>) -> Self {
        let element_layout = alloc::Layout::new::<T>();

        Self {
            inner: RawVecMemory::with_capacity_zeroed_in(capacity, alloc, element_layout),
            _marker: marker::PhantomData,
        }
    }

    pub(crate) unsafe fn into_box(self, len: usize) -> Box<[MaybeUninit<T>], TypedProjAlloc<A>> {
        // Sanity-check one half of the safety requirement (we cannot check the other half).
        debug_assert!(
            len <= self.capacity(),
            "`len` must be smaller than or equal to `self.capacity()`"
        );

        let me = ManuallyDrop::new(self);
        unsafe {
            let slice = ptr::slice_from_raw_parts_mut(me.ptr() as *mut MaybeUninit<T>, len);
            Box::from_raw_in(slice, ptr::read(me.allocator()))
        }
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts_in(ptr: *mut T, capacity: usize, alloc: TypedProjAlloc<A>) -> Self {
        let element_layout = alloc::Layout::new::<T>();

        // SAFETY: Precondition passed to the caller
        unsafe {
            let ptr = ptr.cast();
            let capacity = new_capacity::<T>(capacity);

            Self {
                inner: RawVecMemory::from_raw_parts_in(ptr, capacity, element_layout, alloc),
                _marker: marker::PhantomData,
            }
        }
    }

    #[inline]
    pub(crate) unsafe fn from_non_null_in(ptr: NonNull<T>, capacity: usize, alloc: TypedProjAlloc<A>) -> Self {
        let element_layout = alloc::Layout::new::<T>();

        // SAFETY: Precondition passed to the caller
        unsafe {
            let ptr = ptr.cast();
            let capacity = new_capacity::<T>(capacity);

            Self {
                inner: RawVecMemory::from_non_null_in(ptr, capacity, element_layout, alloc),
                _marker: marker::PhantomData
            }
        }
    }
}

impl<T, A> TypedProjRawVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    #[track_caller]
    pub(crate) fn reserve(&mut self, len: usize, additional: usize) {
        self.inner.reserve(len, additional, alloc::Layout::new::<T>())
    }

    #[inline(never)]
    #[track_caller]
    pub(crate) fn grow_one(&mut self) {
        self.inner.grow_one(alloc::Layout::new::<T>())
    }

    pub(crate) fn try_reserve(
        &mut self,
        len: usize,
        additional: usize,
    ) -> Result<(), TryReserveError> {
        self.inner.try_reserve(len, additional, alloc::Layout::new::<T>())
    }

    #[track_caller]
    pub(crate) fn reserve_exact(&mut self, len: usize, additional: usize) {
        self.inner.reserve_exact(len, additional, alloc::Layout::new::<T>())
    }

    pub(crate) fn try_reserve_exact(
        &mut self,
        len: usize,
        additional: usize,
    ) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(len, additional, alloc::Layout::new::<T>())
    }

    #[track_caller]
    #[inline]
    pub(crate) fn shrink_to_fit(&mut self, capacity: usize) {
        self.inner.shrink_to_fit(capacity, alloc::Layout::new::<T>())
    }
}

impl<T, A> Drop for TypedProjRawVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Frees the memory owned by the `RawVec` *without* trying to drop its contents.
    fn drop(&mut self) {
        // SAFETY: We are in a Drop impl, self.inner will not be used again.
        unsafe { self.inner.deallocate(alloc::Layout::new::<T>()) }
    }
}

#[repr(transparent)]
pub(crate) struct OpaqueRawVec {
    inner: RawVecMemory,
}

impl OpaqueRawVec {
    #[inline(always)]
    pub(crate) fn as_proj_assuming_type<T, A>(&self) -> &TypedProjRawVec<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        unsafe { &*(self as *const OpaqueRawVec as *const TypedProjRawVec<T, A>) }
    }

    #[inline(always)]
    pub(crate) fn as_proj_mut_assuming_type<T, A>(&mut self) -> &mut TypedProjRawVec<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        unsafe { &mut *(self as *mut OpaqueRawVec as *mut TypedProjRawVec<T, A>) }
    }

    #[inline(always)]
    pub(crate) fn into_proj_assuming_type<T, A>(self) -> TypedProjRawVec<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        unsafe { mem::transmute(self) }
    }

    #[inline(always)]
    pub(crate) fn from_proj<T, A>(proj_self: TypedProjRawVec<T, A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        unsafe { mem::transmute(proj_self) }
    }
}

impl OpaqueRawVec {
    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }
}

impl OpaqueRawVec {
    #[inline]
    pub(crate) const fn element_layout(&self) -> alloc::Layout {
        self.inner.element_layout()
    }

    #[inline]
    pub(crate) const fn capacity(&self) -> usize {
        self.inner.capacity(self.inner.element_layout().size())
    }
}

impl OpaqueRawVec {
    #[inline]
    pub(crate) const fn as_non_null(&self) -> NonNull<u8> {
        self.inner.non_null::<u8>()
    }
}

impl Drop for OpaqueRawVec {
    /// Frees the memory owned by the `RawVec` *without* trying to drop its contents.
    fn drop(&mut self) {
        // SAFETY: We are in a Drop impl, self.inner will not be used again.
        unsafe { self.inner.deallocate(self.element_layout()) }
    }
}

struct RawVecMemory {
    ptr: NonNull<u8>,
    capacity: Capacity,
    layout: alloc::Layout,
    alloc: OpaqueAlloc,
}

impl RawVecMemory {
    #[inline]
    const fn allocator_type_id(&self) -> any::TypeId {
        self.alloc.allocator_type_id()
    }
}

impl RawVecMemory {
    #[inline]
    const fn element_layout(&self) -> alloc::Layout {
        self.layout
    }

    #[inline]
    const fn capacity(&self, element_size: usize) -> usize {
        if element_size == 0 { usize::MAX } else { self.capacity.as_inner() }
    }
}

impl RawVecMemory {
    #[inline]
    const fn ptr<T>(&self) -> *mut T {
        self.non_null::<T>().as_ptr()
    }

    #[inline]
    const fn non_null<T>(&self) -> NonNull<T> {
        self.ptr.cast::<T>()
    }

    #[inline]
    fn allocator<A>(&self) -> &TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.alloc.as_proj::<A>()
    }
}

impl RawVecMemory {
    #[inline]
    fn new_in<A>(proj_alloc: TypedProjAlloc<A>, element_layout: alloc::Layout) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let ptr = unsafe { core::mem::transmute(element_layout.align()) };
        let capacity = ZERO_CAPACITY;
        let opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);

        Self {
            ptr,
            capacity,
            layout: element_layout,
            alloc: opaque_alloc,
        }
    }

    fn try_allocate_in<A>(capacity: usize, init: AllocInit, proj_alloc: TypedProjAlloc<A>, element_layout: alloc::Layout) -> Result<Self, TryReserveError>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        use std::alloc::Allocator;

        // We avoid `unwrap_or_else` here because it bloats the amount of
        // LLVM IR generated.
        let layout = match layout_array(capacity, element_layout) {
            Ok(layout) => layout,
            Err(_) => {
                return Err(TryReserveError::from(
                    TryReserveErrorKind::CapacityOverflow,
                ));
            }
        };

        // Don't allocate here because `Drop` will not deallocate when `capacity` is 0.
        if layout.size() == 0 {
            return Ok(Self::new_in(proj_alloc, element_layout));
        }

        if let Err(err) = alloc_guard(layout.size()) {
            return Err(err);
        }

        let result = match init {
            AllocInit::Uninitialized => proj_alloc.allocate(layout),
            AllocInit::Zeroed => proj_alloc.allocate_zeroed(layout),
        };
        let ptr = match result {
            Ok(ptr) => ptr,
            Err(_) => return Err(TryReserveErrorKind::AllocError { layout }.into()),
        };
        let opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);

        // Allocators currently return a `NonNull<[u8]>` whose length
        // matches the size requested. If that ever changes, the capacity
        // here should change to `ptr.len() / mem::size_of::<T>()`.
        Ok(Self {
            ptr: NonNull::from(ptr.cast()),
            capacity: unsafe { Capacity::new_unchecked(capacity) },
            layout: element_layout,
            alloc: opaque_alloc,
        })
    }

    #[inline]
    fn try_with_capacity_in<A>(capacity: usize, proj_alloc: TypedProjAlloc<A>, element_layout: alloc::Layout) -> Result<Self, TryReserveError>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        Self::try_allocate_in(capacity, AllocInit::Uninitialized, proj_alloc, element_layout)
    }

    #[inline]
    #[track_caller]
    fn with_capacity_zeroed_in<A>(capacity: usize, proj_alloc: TypedProjAlloc<A>, element_layout: alloc::Layout) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        match Self::try_allocate_in(capacity, AllocInit::Zeroed, proj_alloc, element_layout) {
            Ok(res) => res,
            Err(err) => handle_error(err),
        }
    }

    #[inline]
    #[track_caller]
    fn with_capacity_in<A>(capacity: usize, proj_alloc: TypedProjAlloc<A>, element_layout: alloc::Layout) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        match Self::try_allocate_in(capacity, AllocInit::Uninitialized, proj_alloc, element_layout) {
            Ok(this) => {
                unsafe {
                    // Make it more obvious that a subsequent Vec::reserve(capacity) will not allocate.
                    core::hint::assert_unchecked(!this.needs_to_grow(0, capacity, element_layout));
                }
                this
            }
            Err(err) => handle_error(err),
        }
    }

    #[inline]
    unsafe fn from_raw_parts_in<A>(ptr: *mut u8, capacity: Capacity, element_layout: alloc::Layout, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);

        Self {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            capacity,
            layout: element_layout,
            alloc: opaque_alloc,
        }
    }

    #[inline]
    unsafe fn from_non_null_in<A>(ptr: NonNull<u8>, capacity: Capacity, element_layout: alloc::Layout, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);

        Self {
            ptr: NonNull::from(ptr),
            capacity,
            layout: element_layout,
            alloc: opaque_alloc,
        }
    }
}

impl RawVecMemory {
    #[inline]
    fn current_memory(&self, element_layout: alloc::Layout) -> Option<(NonNull<u8>, alloc::Layout)> {
        if element_layout.size() == 0 || self.capacity.as_inner() == 0 {
            None
        } else {
            // We could use Layout::array here which ensures the absence of isize and usize overflows
            // and could hypothetically handle differences between stride and size, but this memory
            // has already been allocated so we know it can't overflow and currently Rust does not
            // support such types. So we can do better by skipping some checks and avoid an unwrap.
            unsafe {
                let alloc_size = element_layout.size().unchecked_mul(self.capacity.as_inner());
                let layout = alloc::Layout::from_size_align_unchecked(alloc_size, element_layout.align());

                Some((self.ptr.into(), layout))
            }
        }
    }

    fn try_reserve(
        &mut self,
        len: usize,
        additional: usize,
        element_layout: alloc::Layout,
    ) -> Result<(), TryReserveError> {
        if self.needs_to_grow(len, additional, element_layout) {
            self.grow_amortized(len, additional, element_layout)?;
        }
        unsafe {
            // Inform the optimizer that the reservation has succeeded or wasn't needed
            core::hint::assert_unchecked(!self.needs_to_grow(len, additional, element_layout));
        }
        Ok(())
    }

    fn try_reserve_exact(
        &mut self,
        len: usize,
        additional: usize,
        element_layout: alloc::Layout,
    ) -> Result<(), TryReserveError> {
        if self.needs_to_grow(len, additional, element_layout) {
            self.grow_exact(len, additional, element_layout)?;
        }
        unsafe {
            // Inform the optimizer that the reservation has succeeded or wasn't needed
            core::hint::assert_unchecked(!self.needs_to_grow(len, additional, element_layout));
        }

        Ok(())
    }
    
    #[inline]
    #[track_caller]
    fn reserve(&mut self, len: usize, additional: usize, element_layout: alloc::Layout) {
        // Callers expect this function to be very cheap when there is already sufficient capacity.
        // Therefore, we move all the resizing and error-handling logic from grow_amortized and
        // handle_reserve behind a call, while making sure that this function is likely to be
        // inlined as just a comparison and a call if the comparison fails.
        #[cold]
        fn do_reserve_and_handle(slf: &mut RawVecMemory, len: usize, additional: usize, element_layout: alloc::Layout) {
            if let Err(err) = slf.grow_amortized(len, additional, element_layout) {
                handle_error(err);
            }
        }

        if self.needs_to_grow(len, additional, element_layout) {
            do_reserve_and_handle(self, len, additional, element_layout);
        }
    }

    #[track_caller]
    fn reserve_exact(&mut self, len: usize, additional: usize, element_layout: alloc::Layout) {
        if let Err(err) = self.try_reserve_exact(len, additional, element_layout) {
            handle_error(err);
        }
    }

    #[inline]
    #[track_caller]
    fn shrink_to_fit(&mut self, capacity: usize, element_layout: alloc::Layout) {
        if let Err(err) = self.shrink(capacity, element_layout) {
            handle_error(err);
        }
    }

    #[inline]
    fn needs_to_grow(&self, len: usize, additional: usize, element_layout: alloc::Layout) -> bool {
        additional > self.capacity(element_layout.size()).wrapping_sub(len)
    }

    #[inline]
    unsafe fn set_ptr_and_capacity(&mut self, ptr: NonNull<[u8]>, capacity: usize) {
        // Allocators currently return a `NonNull<[u8]>` whose length matches
        // the size requested. If that ever changes, the capacity here should
        // change to `ptr.len() / mem::size_of::<T>()`.
        self.ptr = NonNull::from(ptr.cast());
        self.capacity = unsafe { Capacity::new_unchecked(capacity) };
    }

    fn grow_amortized(&mut self, length: usize, additional: usize, element_layout: alloc::Layout) -> Result<(), TryReserveError> {
        // This is ensured by the calling contexts.
        debug_assert!(additional > 0);

        if element_layout.size() == 0 {
            // Since we return a capacity of `usize::MAX` when `elem_size` is
            // 0, getting to here necessarily means the `RawVec` is overfull.
            return Err(TryReserveError::from(
                TryReserveErrorKind::CapacityOverflow,
            ));
        }

        // Nothing we can really do about these checks, sadly.
        let required_capacity = length
            .checked_add(additional)
            .ok_or(TryReserveErrorKind::CapacityOverflow)?;

        // This guarantees exponential growth. The doubling cannot overflow
        // because `capacity <= isize::MAX` and the type of `capacity` is `usize`.
        let capacity = core::cmp::max(self.capacity.as_inner() * 2, required_capacity);
        let capacity = core::cmp::max(min_non_zero_capacity(element_layout.size()), capacity);

        let new_layout = layout_array(capacity, element_layout)?;

        let ptr = finish_grow(new_layout, self.current_memory(element_layout), &mut self.alloc)?;
        // SAFETY: finish_grow would have resulted in a capacity overflow if we tried to allocate more than `isize::MAX` items

        unsafe { self.set_ptr_and_capacity(ptr, capacity) };

        Ok(())
    }

    fn grow_exact(&mut self, len: usize, additional: usize, element_layout: alloc::Layout) -> Result<(), TryReserveError> {
        if element_layout.size() == 0 {
            // Since we return a capacity of `usize::MAX` when the type size is
            // 0, getting to here necessarily means the `RawVec` is overfull.
            return Err(TryReserveError::from(
                TryReserveErrorKind::CapacityOverflow,
            ));
        }

        let capacity = len.checked_add(additional).ok_or(TryReserveError::from(
            TryReserveErrorKind::CapacityOverflow,
        ))?;
        let new_layout = layout_array(capacity, element_layout)?;

        let ptr = finish_grow(new_layout, self.current_memory(element_layout), &mut self.alloc)?;
        // SAFETY: finish_grow would have resulted in a capacity overflow if we tried to allocate more than `isize::MAX` items
        unsafe {
            self.set_ptr_and_capacity(ptr, capacity);
        }
        Ok(())
    }

    #[inline]
    #[track_caller]
    fn grow_one(&mut self, element_layout: alloc::Layout) {
        if let Err(err) = self.grow_amortized(self.capacity.as_inner(), 1, element_layout) {
            match err.kind() {
                TryReserveErrorKind::CapacityOverflow => capacity_overflow(),
                TryReserveErrorKind::AllocError { layout, .. } => alloc::handle_alloc_error(layout),
            }
        }
    }

    /// `shrink`, but without the capacity check.
    ///
    /// This is split out so that `shrink` can inline the check, since it
    /// optimizes out in things like `shrink_to_fit`, without needing to
    /// also inline all this code, as doing that ends up failing the
    /// `vec-shrink-panic` codegen test when `shrink_to_fit` ends up being too
    /// big for LLVM to be willing to inline.
    ///
    /// # Safety
    /// `capacity <= self.capacity()`
    unsafe fn shrink_unchecked(&mut self, capacity: usize, element_layout: alloc::Layout) -> Result<(), TryReserveError> {
        use std::alloc::Allocator;

        let (ptr, layout) = if let Some(mem) = self.current_memory(element_layout) {
            mem
        } else {
            return Ok(());
        };

        // If shrinking to 0, deallocate the buffer. We don't reach this point
        // for the is_zst<T>() case since current_memory() will have returned
        // None.
        if capacity == 0 {
            unsafe { self.alloc.deallocate(ptr, layout) };
            self.ptr = unsafe { NonNull::new_unchecked(ptr::without_provenance_mut(element_layout.align())) };
            self.capacity = ZERO_CAPACITY;
        } else {
            let ptr = unsafe {
                // Layout cannot overflow here because it would have
                // overflowed earlier when capacity was larger.
                let new_size = element_layout.size().unchecked_mul(capacity);
                let new_layout = alloc::Layout::from_size_align_unchecked(new_size, layout.align());
                self.alloc
                    .shrink(ptr, layout, new_layout)
                    .map_err(|_| TryReserveErrorKind::AllocError { layout: new_layout })?
            };
            // SAFETY: if the allocation is valid, then the capacity is too
            unsafe {
                self.set_ptr_and_capacity(ptr, capacity);
            }
        }

        Ok(())
    }

    #[inline]
    fn shrink(&mut self, capacity: usize, element_layout: alloc::Layout) -> Result<(), TryReserveError> {
        assert!(
            capacity <= self.capacity(element_layout.size()),
            "Tried to shrink to a larger capacity"
        );
        // SAFETY: Just checked this isn't trying to grow
        unsafe { self.shrink_unchecked(capacity, element_layout) }
    }

    /// # Safety
    ///
    /// This function deallocates the owned allocation, but does not update `ptr` or `capacity` to
    /// prevent double-free or use-after-free. Essentially, do not do anything with the caller
    /// after this function returns.
    /// Ideally this function would take `self` by move, but it cannot because it exists to be
    /// called from a `Drop` impl.
    unsafe fn deallocate(&mut self, element_layout: alloc::Layout) {
        if let Some((ptr, layout)) = self.current_memory(element_layout) {
            unsafe {
                alloc::Allocator::deallocate(&mut self.alloc, ptr, layout);
            }
        }
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

mod layout_testing_types {
    use super::*;
    use core::marker;

    #[allow(dead_code)]
    pub(super) struct TangentSpace {
        tangent: [f32; 3],
        bitangent: [f32; 3],
        normal: [f32; 3],
        _do_not_construct: marker::PhantomData<()>,
    }

    #[allow(dead_code)]
    pub(super) struct SurfaceDifferential {
        dpdu: [f32; 3],
        dpdv: [f32; 3],
        _do_not_construct: marker::PhantomData<()>,
    }

    #[allow(dead_code)]
    pub(super) struct OctTreeNode {
        center: [f32; 3],
        extent: f32,
        children: [Option<Box<OctTreeNode>>; 8],
        occupancy: u8,
        _do_not_construct: marker::PhantomData<()>,
    }
}

#[cfg(test)]
mod raw_vec_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_raw_vec_match_sizes<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::size_of::<TypedProjRawVec<T, A>>();
        let result = mem::size_of::<OpaqueRawVec>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_raw_vec_match_alignments<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::align_of::<TypedProjRawVec<T, A>>();
        let result = mem::align_of::<OpaqueRawVec>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_raw_vec_match_offsets<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::offset_of!(TypedProjRawVec<T, A>, inner);
        let result = mem::offset_of!(OpaqueRawVec, inner);

        assert_eq!(result, expected, "Opaque and Typed Projected data types offsets mismatch");
    }

    macro_rules! layout_tests {
        ($module_name:ident, $element_typ:ty, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_opaque_raw_vec_layout_match_sizes() {
                    run_test_opaque_raw_vec_match_sizes::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_raw_vec_layout_match_alignments() {
                    run_test_opaque_raw_vec_match_alignments::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_raw_vec_layout_match_offsets() {
                    run_test_opaque_raw_vec_match_offsets::<$element_typ, $alloc_typ>();
                }
            }
        };
    }

    layout_tests!(unit_zst_global, (), alloc::Global);
    layout_tests!(u8_global, u8, alloc::Global);
    layout_tests!(u16_global, u16, alloc::Global);
    layout_tests!(u32_global, u32, alloc::Global);
    layout_tests!(u64_global, u64, alloc::Global);
    layout_tests!(tangent_space_global, layout_testing_types::TangentSpace, alloc::Global);
    layout_tests!(surface_differential_global, layout_testing_types::SurfaceDifferential, alloc::Global);
    layout_tests!(oct_tree_node_global, layout_testing_types::OctTreeNode, alloc::Global);

    layout_tests!(unit_zst_dummy_alloc, (), dummy::DummyAlloc);
    layout_tests!(u8_dummy_alloc,  u8, dummy::DummyAlloc);
    layout_tests!(u16_dummy_alloc, u16, dummy::DummyAlloc);
    layout_tests!(u32_dummy_alloc, u32, dummy::DummyAlloc);
    layout_tests!(u64_dummy_alloc, u64, dummy::DummyAlloc);
    layout_tests!(tangent_space_dummy_alloc, layout_testing_types::TangentSpace, dummy::DummyAlloc);
    layout_tests!(surface_differential_dummy_alloc, layout_testing_types::SurfaceDifferential, dummy::DummyAlloc);
    layout_tests!(oct_tree_node_dummy_alloc, layout_testing_types::OctTreeNode, dummy::DummyAlloc);
}

#[cfg(test)]
mod assert_send_sync {
    use crate::TypedProjVec;
    use super::*;

    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjRawVec<i32, alloc::Global>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjVec<i32, dummy::DummyAlloc>>();
    }
}
