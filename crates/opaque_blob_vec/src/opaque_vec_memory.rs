use core::ptr::NonNull;
use std::alloc::{
    Allocator,
    Global,
    Layout,
};

use crate::range_types::UsizeNoHighBit;
use crate::unique::Unique;

use opaque_error;

// One central function responsible for reporting capacity overflows. This will
// ensure that the code generation related to these panics is minimal as there's
// only one location which panics rather than a bunch throughout the module.
#[cfg(not(no_global_oom_handling))]
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never))]
#[track_caller]
fn capacity_overflow() -> ! {
    panic!("capacity overflow");
}

enum AllocInit {
    /// The contents of the new memory are uninitialized.
    Uninitialized,
    #[cfg(not(no_global_oom_handling))]
    /// The new memory is guaranteed to be zeroed.
    Zeroed,
}

type Capacity = UsizeNoHighBit;

const ZERO_CAP: Capacity = unsafe { Capacity::new_unchecked(0) };

pub(crate) unsafe fn new_capacity(capacity: usize, layout: Layout) -> Capacity {
    if layout.size() == 0 {
        ZERO_CAP
    } else {
        unsafe { Capacity::new_unchecked(capacity) }
    }
}

// Central function for reserve error handling.
#[cold]
#[optimize(size)]
#[track_caller]
fn handle_error(err: opaque_error::TryReserveError) -> ! {
    match err.kind() {
        opaque_error::TryReserveErrorKind::CapacityOverflow => capacity_overflow(),
        opaque_error::TryReserveErrorKind::AllocError { layout, .. } => std::alloc::handle_alloc_error(layout),
    }
}

#[inline]
fn layout_array(capacity: usize, element_layout: Layout) -> Result<Layout, opaque_error::TryReserveError> {
    element_layout
        .repeat(capacity)
        .map(|(layout, _pad)| layout)
        .map_err(|_| opaque_error::TryReserveError::from(opaque_error::TryReserveErrorKind::CapacityOverflow))
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
fn alloc_guard(alloc_size: usize) -> Result<(), opaque_error::TryReserveError> {
    if usize::BITS < 64 && alloc_size > isize::MAX as usize {
        Err(opaque_error::TryReserveError::from(
            opaque_error::TryReserveErrorKind::CapacityOverflow,
        ))
    } else {
        Ok(())
    }
}

// not marked inline(never) since we want optimizers to be able to observe the specifics of this
// function, see tests/codegen/vec-reserve-extend.rs.
#[cold]
fn finish_grow<A>(
    new_layout: Layout,
    current_memory: Option<(NonNull<u8>, Layout)>,
    alloc: &mut A,
) -> Result<NonNull<[u8]>, opaque_error::TryReserveError>
where
    A: std::alloc::Allocator,
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

    memory.map_err(|_| opaque_error::TryReserveError::from(opaque_error::TryReserveErrorKind::AllocError { layout: new_layout }))
}


pub(crate) struct OpaqueVecMemory<A: Allocator = Global> {
    ptr: Unique<u8>,
    capacity: Capacity,
    alloc: A,
}

impl OpaqueVecMemory {
    #[must_use]
    pub(crate) const fn new(element_layout: Layout) -> Self {
        Self::new_in(Global, element_layout)
    }

    #[must_use]
    #[inline]
    #[track_caller]
    pub(crate) fn with_capacity(capacity: usize, element_layout: Layout) -> Self {
        match Self::try_allocate_in(capacity, AllocInit::Uninitialized, Global, element_layout) {
            Ok(res) => res,
            Err(err) => handle_error(err),
        }
    }
}

impl<A> OpaqueVecMemory<A>
where
    A: Allocator,
{
    #[inline]
    pub(crate) const fn new_in(alloc: A, element_layout: Layout) -> Self {
        let ptr = unsafe { core::mem::transmute(element_layout.align()) };
        let capacity = ZERO_CAP;

        Self { ptr, capacity, alloc }
    }

    fn try_allocate_in(capacity: usize, init: AllocInit, alloc: A, element_layout: Layout) -> Result<Self, opaque_error::TryReserveError> {
        // We avoid `unwrap_or_else` here because it bloats the amount of
        // LLVM IR generated.
        let layout = match layout_array(capacity, element_layout) {
            Ok(layout) => layout,
            Err(_) => {
                return Err(opaque_error::TryReserveError::from(
                    opaque_error::TryReserveErrorKind::CapacityOverflow,
                ));
            }
        };

        // Don't allocate here because `Drop` will not deallocate when `capacity` is 0.
        if layout.size() == 0 {
            return Ok(Self::new_in(alloc, element_layout));
        }

        if let Err(err) = alloc_guard(layout.size()) {
            return Err(err);
        }

        let result = match init {
            AllocInit::Uninitialized => alloc.allocate(layout),
            #[cfg(not(no_global_oom_handling))]
            AllocInit::Zeroed => alloc.allocate_zeroed(layout),
        };
        let ptr = match result {
            Ok(ptr) => ptr,
            Err(_) => return Err(opaque_error::TryReserveErrorKind::AllocError { layout }.into()),
        };

        // Allocators currently return a `NonNull<[u8]>` whose length
        // matches the size requested. If that ever changes, the capacity
        // here should change to `ptr.len() / mem::size_of::<T>()`.
        Ok(Self {
            ptr: Unique::from(ptr.cast()),
            capacity: unsafe { Capacity::new_unchecked(capacity) },
            alloc,
        })
    }

    #[inline]
    pub(crate) fn try_with_capacity_in(capacity: usize, alloc: A, element_layout: Layout) -> Result<Self, opaque_error::TryReserveError> {
        Self::try_allocate_in(capacity, AllocInit::Uninitialized, alloc, element_layout)
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    fn with_capacity_zeroed_in(capacity: usize, alloc: A, element_layout: Layout) -> Self {
        match Self::try_allocate_in(capacity, AllocInit::Zeroed, alloc, element_layout) {
            Ok(res) => res,
            Err(err) => handle_error(err),
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub(crate) fn with_capacity_in(capacity: usize, alloc: A, element_layout: Layout) -> Self {
        match Self::try_allocate_in(capacity, AllocInit::Uninitialized, alloc, element_layout) {
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
    pub(crate) unsafe fn from_raw_parts_in(ptr: *mut u8, capacity: Capacity, alloc: A) -> Self {
        Self {
            ptr: unsafe { Unique::new_unchecked(ptr) },
            capacity,
            alloc,
        }
    }

    #[inline]
    pub(crate) unsafe fn from_nonnull_in(ptr: NonNull<u8>, capacity: Capacity, alloc: A) -> Self {
        Self {
            ptr: Unique::from(ptr),
            capacity,
            alloc,
        }
    }

    #[inline]
    pub(crate) const fn ptr<T>(&self) -> *mut T {
        self.non_null::<T>().as_ptr()
    }

    #[inline]
    const fn non_null<T>(&self) -> NonNull<T> {
        self.ptr.cast().as_non_null_ptr()
    }

    #[inline]
    pub(crate) const fn capacity(&self, element_size: usize) -> usize {
        if element_size == 0 { usize::MAX } else { self.capacity.as_inner() }
    }

    #[inline]
    pub(crate) const fn allocator(&self) -> &A {
        &self.alloc
    }

    #[inline]
    fn current_memory(&self, element_layout: Layout) -> Option<(NonNull<u8>, Layout)> {
        if element_layout.size() == 0 || self.capacity.as_inner() == 0 {
            None
        } else {
            // We could use Layout::array here which ensures the absence of isize and usize overflows
            // and could hypothetically handle differences between stride and size, but this memory
            // has already been allocated so we know it can't overflow and currently Rust does not
            // support such types. So we can do better by skipping some checks and avoid an unwrap.
            unsafe {
                let alloc_size = element_layout.size().unchecked_mul(self.capacity.as_inner());
                let layout = Layout::from_size_align_unchecked(alloc_size, element_layout.align());

                Some((self.ptr.into(), layout))
            }
        }
    }

    pub(crate) fn try_reserve(
        &mut self,
        len: usize,
        additional: usize,
        element_layout: Layout,
    ) -> Result<(), opaque_error::TryReserveError> {
        if self.needs_to_grow(len, additional, element_layout) {
            self.grow_amortized(len, additional, element_layout)?;
        }
        unsafe {
            // Inform the optimizer that the reservation has succeeded or wasn't needed
            core::hint::assert_unchecked(!self.needs_to_grow(len, additional, element_layout));
        }
        Ok(())
    }

    pub(crate) fn try_reserve_exact(
        &mut self,
        len: usize,
        additional: usize,
        element_layout: Layout,
    ) -> Result<(), opaque_error::TryReserveError> {
        if self.needs_to_grow(len, additional, element_layout) {
            self.grow_exact(len, additional, element_layout)?;
        }
        unsafe {
            // Inform the optimizer that the reservation has succeeded or wasn't needed
            core::hint::assert_unchecked(!self.needs_to_grow(len, additional, element_layout));
        }

        Ok(())
    }


    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub(crate) fn reserve(&mut self, len: usize, additional: usize, element_layout: Layout) {
        // Callers expect this function to be very cheap when there is already sufficient capacity.
        // Therefore, we move all the resizing and error-handling logic from grow_amortized and
        // handle_reserve behind a call, while making sure that this function is likely to be
        // inlined as just a comparison and a call if the comparison fails.
        #[cold]
        fn do_reserve_and_handle<A: Allocator>(slf: &mut OpaqueVecMemory<A>, len: usize, additional: usize, element_layout: Layout) {
            if let Err(err) = slf.grow_amortized(len, additional, element_layout) {
                handle_error(err);
            }
        }

        if self.needs_to_grow(len, additional, element_layout) {
            do_reserve_and_handle(self, len, additional, element_layout);
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub(crate) fn reserve_exact(&mut self, len: usize, additional: usize, element_layout: Layout) {
        if let Err(err) = self.try_reserve_exact(len, additional, element_layout) {
            handle_error(err);
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub(crate) fn shrink_to_fit(&mut self, capacity: usize, element_layout: Layout) {
        if let Err(err) = self.shrink(capacity, element_layout) {
            handle_error(err);
        }
    }

    #[inline]
    fn needs_to_grow(&self, len: usize, additional: usize, element_layout: Layout) -> bool {
        additional > self.capacity(element_layout.size()).wrapping_sub(len)
    }

    #[inline]
    unsafe fn set_ptr_and_cap(&mut self, ptr: NonNull<[u8]>, capacity: usize) {
        // Allocators currently return a `NonNull<[u8]>` whose length matches
        // the size requested. If that ever changes, the capacity here should
        // change to `ptr.len() / mem::size_of::<T>()`.
        self.ptr = Unique::from(ptr.cast());
        self.capacity = unsafe { Capacity::new_unchecked(capacity) };
    }

    fn grow_amortized(&mut self, length: usize, additional: usize, element_layout: Layout) -> Result<(), opaque_error::TryReserveError> {
        const fn min_non_zero_cap(size: usize) -> usize {
            if size == 1 {
                8
            } else if size <= 1024 {
                4
            } else {
                1
            }
        }

        // This is ensured by the calling contexts.
        debug_assert!(additional > 0);

        if element_layout.size() == 0 {
            // Since we return a capacity of `usize::MAX` when `elem_size` is
            // 0, getting to here necessarily means the `RawVec` is overfull.
            return Err(opaque_error::TryReserveError::from(
                opaque_error::TryReserveErrorKind::CapacityOverflow,
            ));
        }

        // Nothing we can really do about these checks, sadly.
        let required_capacity = length
            .checked_add(additional)
            .ok_or(opaque_error::TryReserveErrorKind::CapacityOverflow)?;

        // This guarantees exponential growth. The doubling cannot overflow
        // because `cap <= isize::MAX` and the type of `cap` is `usize`.
        let capacity = std::cmp::max(self.capacity.as_inner() * 2, required_capacity);
        let capacity = std::cmp::max(min_non_zero_cap(element_layout.size()), capacity);

        let new_layout = layout_array(capacity, element_layout)?;

        let ptr = finish_grow(new_layout, self.current_memory(element_layout), &mut self.alloc)?;
        // SAFETY: finish_grow would have resulted in a capacity overflow if we tried to allocate more than `isize::MAX` items

        unsafe { self.set_ptr_and_cap(ptr, capacity) };

        Ok(())
    }

    fn grow_exact(&mut self, len: usize, additional: usize, element_layout: Layout) -> Result<(), opaque_error::TryReserveError> {
        if element_layout.size() == 0 {
            // Since we return a capacity of `usize::MAX` when the type size is
            // 0, getting to here necessarily means the `RawVec` is overfull.
            return Err(opaque_error::TryReserveError::from(
                opaque_error::TryReserveErrorKind::CapacityOverflow,
            ));
        }

        let cap = len.checked_add(additional).ok_or(opaque_error::TryReserveError::from(
            opaque_error::TryReserveErrorKind::CapacityOverflow,
        ))?;
        let new_layout = layout_array(cap, element_layout)?;

        let ptr = finish_grow(new_layout, self.current_memory(element_layout), &mut self.alloc)?;
        // SAFETY: finish_grow would have resulted in a capacity overflow if we tried to allocate more than `isize::MAX` items
        unsafe {
            self.set_ptr_and_cap(ptr, cap);
        }
        Ok(())
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub(crate) fn grow_one(&mut self, element_layout: Layout) {
        if let Err(err) = self.grow_amortized(self.capacity.as_inner(), 1, element_layout) {
            match err.kind() {
                opaque_error::TryReserveErrorKind::CapacityOverflow => capacity_overflow(),
                opaque_error::TryReserveErrorKind::AllocError { layout, .. } => std::alloc::handle_alloc_error(layout),
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
    /// `cap <= self.capacity()`
    #[cfg(not(no_global_oom_handling))]
    unsafe fn shrink_unchecked(&mut self, capacity: usize, element_layout: Layout) -> Result<(), opaque_error::TryReserveError> {
        let (ptr, layout) = if let Some(mem) = self.current_memory(element_layout) {
            mem
        } else {
            return Ok(());
        };

        // If shrinking to 0, deallocate the buffer. We don't reach this point
        // for the T::IS_ZST case since current_memory() will have returned
        // None.
        if capacity == 0 {
            unsafe { self.alloc.deallocate(ptr, layout) };
            self.ptr = unsafe { Unique::new_unchecked(std::ptr::without_provenance_mut(element_layout.align())) };
            self.capacity = ZERO_CAP;
        } else {
            let ptr = unsafe {
                // Layout cannot overflow here because it would have
                // overflowed earlier when capacity was larger.
                let new_size = element_layout.size().unchecked_mul(capacity);
                let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
                self.alloc
                    .shrink(ptr, layout, new_layout)
                    .map_err(|_| opaque_error::TryReserveErrorKind::AllocError { layout: new_layout })?
            };
            // SAFETY: if the allocation is valid, then the capacity is too
            unsafe {
                self.set_ptr_and_cap(ptr, capacity);
            }
        }

        Ok(())
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    fn shrink(&mut self, capacity: usize, element_layout: Layout) -> Result<(), opaque_error::TryReserveError> {
        assert!(
            capacity <= self.capacity(element_layout.size()),
            "Tried to shrink to a larger capacity"
        );
        // SAFETY: Just checked this isn't trying to grow
        unsafe { self.shrink_unchecked(capacity, element_layout) }
    }

    /// # Safety
    ///
    /// This function deallocates the owned allocation, but does not update `ptr` or `cap` to
    /// prevent double-free or use-after-free. Essentially, do not do anything with the caller
    /// after this function returns.
    /// Ideally this function would take `self` by move, but it cannot because it exists to be
    /// called from a `Drop` impl.
    pub(crate) unsafe fn deallocate(&mut self, element_layout: Layout) {
        if let Some((ptr, layout)) = self.current_memory(element_layout) {
            unsafe {
                self.alloc.deallocate(ptr, layout);
            }
        }
    }
}
