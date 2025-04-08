use std::alloc::Layout;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;
use opaque_alloc::OpaqueAlloc;
use crate::opaque_vec_memory;
use crate::opaque_vec_memory::OpaqueVecMemory;
use crate::try_reserve_error::TryReserveError;

pub(crate) struct OpaqueVecInner {
    element_layout: Layout,
    length: usize,
    data: OpaqueVecMemory<OpaqueAlloc>,
    drop_fn: Option<unsafe fn(NonNull<u8>)>,
}

impl OpaqueVecInner {
    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) const fn new_in(alloc: OpaqueAlloc, element_layout: Layout, drop_fn: Option<unsafe fn(NonNull<u8>)>) -> Self {
        let length = 0;
        let data = OpaqueVecMemory::new_in(alloc, element_layout);

        Self {
            element_layout,
            length,
            data,
            drop_fn,
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity_in(capacity: usize, alloc: OpaqueAlloc, element_layout: Layout, drop_fn: Option<unsafe fn(NonNull<u8>)>) -> Self {
        let length = 0;
        let data = OpaqueVecMemory::with_capacity_in(capacity, alloc, element_layout);

        Self {
            element_layout,
            length,
            data,
            drop_fn,
        }
    }

    #[inline]
    pub(crate) fn try_with_capacity_in(
        capacity: usize,
        alloc: OpaqueAlloc,
        element_layout: Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>
    ) -> Result<Self, TryReserveError>
    {
        let length = 0;
        let data = OpaqueVecMemory::try_with_capacity_in(capacity, alloc, element_layout)?;
        let vec = Self {
            element_layout,
            length,
            data,
            drop_fn,
        };

        Ok(vec)
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts_in(
        ptr: *mut u8,
        length: usize,
        capacity: usize,
        alloc: OpaqueAlloc,
        element_layout: Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>
    ) -> Self
    {
        let capacity_bytes = opaque_vec_memory::new_capacity(capacity, element_layout);
        let data = OpaqueVecMemory::from_raw_parts_in(ptr, capacity_bytes, alloc);

        Self {
            element_layout,
            length,
            data,
            drop_fn,
        }
    }

    #[inline]
    pub unsafe fn from_parts_in(
        ptr: NonNull<u8>,
        length: usize,
        capacity: usize,
        alloc: OpaqueAlloc,
        element_layout: Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>
    ) -> Self
    {
        let capacity_bytes = opaque_vec_memory::new_capacity(capacity, element_layout);
        let data = OpaqueVecMemory::from_nonnull_in(ptr, capacity_bytes, alloc);

        Self {
            element_layout,
            length,
            data,
            drop_fn,
        }
    }

    #[inline]
    pub(crate) const fn allocator(&self) -> &OpaqueAlloc {
        self.data.allocator()
    }
}

impl OpaqueVecInner {
    #[inline]
    pub(crate) const fn element_layout(&self) -> Layout {
        self.element_layout
    }

    #[inline]
    pub(crate) const fn capacity(&self) -> usize {
        self.data.capacity(self.element_layout.size())
    }

    #[inline]
    pub(crate) const fn len(&self) -> usize {
        self.length
    }

    #[inline]
    pub(crate) const fn is_empty(&self) -> bool {
        self.length == 0
    }

    #[inline]
    pub(crate) fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        self.length = new_len;
    }

    #[inline]
    pub(crate) const fn as_ptr(&self) -> *const u8 {
        self.data.ptr() as *const u8
    }

    #[inline]
    pub(crate) const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.ptr()
    }

    #[inline]
    pub const fn as_non_null(&mut self) -> NonNull<u8> {
        // SAFETY: A `Vec` always holds a non-null pointer.
        unsafe {
            NonNull::new_unchecked(self.as_mut_ptr())
        }
    }

    #[inline]
    pub(crate) fn grow_one(&mut self) {
        self.data.grow_one(self.element_layout);
    }

    pub(crate) fn get_unchecked(&self, index: usize) -> NonNull<u8> {
        let base_ptr = self.as_ptr();
        let element = unsafe {
            let ptr = base_ptr.add(index * self.element_layout.size());
            NonNull::new_unchecked(ptr as *mut u8)
        };

        element
    }

    pub(crate) fn get_mut_unchecked(&mut self, index: usize) -> NonNull<u8> {
        let base_ptr = self.as_mut_ptr();
        let element = unsafe {
            let ptr = base_ptr.add(index * self.element_layout.size());
            NonNull::new_unchecked(ptr)
        };

        element
    }

    pub(crate) fn push(&mut self, value: NonNull<u8>) {
        let length = self.len();

        if length == self.capacity() {
            self.data.grow_one(self.element_layout);
        }

        let element_size = self.element_layout().size();

        unsafe {
            let buffer_end = self.as_mut_ptr().add(element_size * length);
            core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), buffer_end, element_size);
        }

        self.length = length + 1;
    }

    pub(crate) fn as_byte_slice(&self) -> &[u8] {
        if self.is_empty() {
            return &[];
        }

        let slice = unsafe {
            let data_ptr = self.as_ptr();
            let len = self.element_layout().size() * self.len();

            std::slice::from_raw_parts(data_ptr, len)
        };

        slice
    }

    #[must_use]
    pub(crate) fn swap_remove_forget_unchecked(&mut self, index: usize) -> NonNull<u8> {
        debug_assert!(index < self.len());

        let new_length = self.length - 1;
        let element_size = self.element_layout.size();
        if index != new_length {
            unsafe {
                core::ptr::swap_nonoverlapping::<u8>(
                    self.get_mut_unchecked(index).as_ptr(),
                    self.get_mut_unchecked(new_length).as_ptr(),
                    element_size,
                );
            }
        }

        self.length = new_length;

        let ptr = unsafe { self.as_mut_ptr().byte_add(element_size * new_length) };

        unsafe {
            NonNull::new_unchecked(ptr)
        }
    }

    #[must_use]
    pub(crate) fn shift_remove_forget_unchecked(&mut self, index: usize) -> NonNull<u8> {
        debug_assert!(index < self.len());

        let new_length = self.length - 1;
        let element_size = self.element_layout.size();
        for i in index..new_length {
            unsafe {
                core::ptr::swap_nonoverlapping::<u8>(
                    self.get_mut_unchecked(i).as_ptr(),
                    self.get_mut_unchecked(i + 1).as_ptr(),
                    element_size,
                )
            }
        }

        self.length = new_length;

        let ptr = unsafe { self.as_mut_ptr().byte_add(element_size * new_length) };

        unsafe {
            NonNull::new_unchecked(ptr)
        }
    }

    pub(crate) fn replace_insert(&mut self, index: usize, value: NonNull<u8>) {
        struct DropGuard<F: FnOnce()> {
            callback: ManuallyDrop<F>,
        }

        impl<F: FnOnce()> DropGuard<F> {
            fn new(callback: F) -> Self {
                Self {
                    callback: ManuallyDrop::new(callback),
                }
            }
        }

        impl<F: FnOnce()> Drop for DropGuard<F> {
            fn drop(&mut self) {
                let callback = unsafe { ManuallyDrop::take(&mut self.callback) };

                callback();
            }
        }

        debug_assert!(index <= self.len());

        let element_size = self.element_layout.size();
        let length = self.len();

        if length == self.capacity() {
            self.grow_one();
        }

        unsafe {
            if index < length {
                let replaced_ptr = NonNull::new_unchecked(self.as_mut_ptr().add(element_size * index));

                if let Some(drop_fn) = self.drop_fn {
                    let old_length = self.length;
                    self.length = 0;

                    let guard = DropGuard::new(|| { drop_fn(value) });

                    drop_fn(replaced_ptr);

                    // We successfully called drop on the value to be replaced, so we no longer
                    // need the `value` guard.
                    core::mem::forget(guard);

                    // Restore the old length.
                    self.length = old_length;
                }

                // SAFETY: The old element has been dropped, so no memory leak will occur with just copying the new
                // value into memory.
                core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), replaced_ptr.as_ptr(), element_size);
            } else {
                let replaced_ptr = NonNull::new_unchecked(self.as_mut_ptr().add(element_size * index));

                // SAFETY: We are pushing to the end of the vector, so no dropping is needed.
                core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), replaced_ptr.as_ptr(), element_size);

                // We pushed to the vec instead of replacing a value inside the vec.
                self.length += 1;
            }
        }
    }

    pub(crate) fn shift_insert(&mut self, index: usize, value: NonNull<u8>) {
        debug_assert!(index <= self.len());

        let length = self.len();
        if length == self.capacity() {
            self.grow_one();
        }

        unsafe {
            {
                let element_size = self.element_layout.size();
                let byte_index = element_size * index;
                let base_ptr = self.as_mut_ptr().add(byte_index);
                if index < length {
                    let byte_count = element_size * (length - index);

                    core::ptr::copy(base_ptr, base_ptr.add(element_size), byte_count);
                }

                core::ptr::copy(value.as_ptr(), base_ptr, element_size);
            }
            self.set_len(length + 1);
        }
    }

    #[inline]
    pub(crate) fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.data.try_reserve(self.length, additional, self.element_layout)
    }

    #[inline]
    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.data.try_reserve_exact(self.length, additional, self.element_layout)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub(crate) fn reserve(&mut self, additional: usize) {
        self.data.reserve(self.length, additional, self.element_layout);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(self.length, additional, self.element_layout);
    }

    #[track_caller]
    #[inline]
    pub(crate) fn shrink_to_fit(&mut self) {
        if self.capacity() > self.length {
            self.data.shrink_to_fit(self.length, self.element_layout);
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub(crate) fn shrink_to(&mut self, min_capacity: usize) {
        if self.capacity() > min_capacity {
            self.data.shrink_to_fit(std::cmp::max(self.length, min_capacity), self.element_layout);
        }
    }

    pub(crate) fn clear(&mut self) {
        let len = self.length;
        self.length = 0;

        if let Some(drop_fn) = self.drop_fn {
            let size = self.element_layout.size();
            let ptr = self.as_non_null();
            for i in 0..len {
                let element = unsafe { ptr.byte_add(i * size) };
                unsafe {
                    drop_fn(element);
                }
            }
        }
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        unsafe {
            if len > self.len() {
                return;
            }

            let remaining_len = self.len() - len;
            let element_size = len * self.element_layout.size();
            let len_bytes = element_size * len;
            let remaining_len_bytes = element_size * remaining_len;
            let slice = core::ptr::slice_from_raw_parts_mut(self.as_mut_ptr().add(len_bytes), remaining_len_bytes);
            self.set_len(len);

            core::ptr::drop_in_place(slice);
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub(crate) fn extend_with(&mut self, count: usize, value: NonNull<u8>) {
        struct SetLenOnDrop<'a> {
            len: &'a mut usize,
            local_len: usize,
        }

        impl<'a> SetLenOnDrop<'a> {
            #[inline]
            fn new(len: &'a mut usize) -> Self {
                SetLenOnDrop { local_len: *len, len }
            }

            #[inline]
            fn increment_len(&mut self, increment: usize) {
                self.local_len += increment;
            }

            #[inline]
            fn current_len(&self) -> usize {
                self.local_len
            }
        }

        impl Drop for SetLenOnDrop<'_> {
            #[inline]
            fn drop(&mut self) {
                *self.len = self.local_len;
            }
        }

        self.reserve(count);

        unsafe {
            let element_size = self.element_layout.size();
            let len_bytes = element_size * self.len();
            let mut ptr = self.as_mut_ptr().add(len_bytes);

            // Use SetLenOnDrop to work around bug where compiler
            // might not realize the store through `ptr` through self.set_len()
            // don't alias.
            let mut local_len = SetLenOnDrop::new(&mut self.length);

            // Write all elements except the last one
            for _ in 1..count {
                core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), ptr, element_size);
                ptr = ptr.add(element_size);
                // Increment the length in every step in case clone() panics
                local_len.increment_len(1);
            }

            if count > 0 {
                // We can write the last element directly without cloning needlessly
                core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), ptr, element_size);
                local_len.increment_len(1);
            }

            // len set by scope guard
        }
    }

    /*
    pub(crate) fn extend_from_iter<I>(&mut self, mut iterator: I)
    where
        I: Iterator<Item = NonNull<u8>>,
    {
        while let Some(element) = iterator.next() {
            let len = self.len();
            if len == self.capacity() {
                let (lower, _) = iterator.size_hint();
                self.reserve(lower.saturating_add(1));
            }

            unsafe {
                let element_size = self.element_layout.size();
                let ptr = self.as_mut_ptr().add(element_size * len);
                core::ptr::copy_nonoverlapping::<u8>(element.as_ptr(), ptr, element_size);
                // Since next() executes user code which can panic we have to bump the length
                // after each step.
                // NB can't overflow since we would have had to alloc the address space
                self.set_len(len + 1);
            }
        }
    }
     */
}

impl Clone for OpaqueVecInner {
    fn clone(&self) -> Self {
        let new_element_layout = self.element_layout;
        let new_length = self.length;
        let new_alloc = self.data.allocator().clone();
        let new_data = OpaqueVecMemory::with_capacity_in(self.capacity(), new_alloc, self.element_layout);
        let new_drop_fn = self.drop_fn.clone();

        unsafe {
            core::ptr::copy_nonoverlapping::<u8>(
                self.data.ptr::<u8>(),
                new_data.ptr::<u8>(),
                new_length * new_element_layout.size(),
            );
        }

        let new_vec = Self {
            element_layout: new_element_layout,
            length: new_length,
            data: new_data,
            drop_fn: new_drop_fn,
        };

        new_vec
    }
}

impl Drop for OpaqueVecInner {
    fn drop(&mut self) {
        self.clear();

        unsafe {
            self.data.deallocate(self.element_layout);
        }
    }
}

#[cfg(test)]
mod opaque_vec_inner_replace_insert_drop_tests {
    use std::alloc::Layout;
    use super::{OpaqueAlloc, OpaqueVecInner};
    use std::panic::{self, AssertUnwindSafe};
    use std::mem::ManuallyDrop;

    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ptr::NonNull;

    #[derive(Clone, Debug)]
    struct DropCounter {
        count: Rc<RefCell<u32>>,
    }

    impl DropCounter {
        #[inline]
        const fn new(count: Rc<RefCell<u32>>) -> Self {
            Self { count }
        }

        fn increment(&mut self) {
            *self.count.borrow_mut() += 1;
        }

        fn drop_count(&self) -> u32 {
            self.count.borrow().clone()
        }
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            *self.count.borrow_mut() += 1;
        }
    }

    #[derive(Clone, Debug)]
    struct PanicCell<T> {
        value: T,
        max_drop_count: u32,
        drop_counter: DropCounter,
        panic_enabled: Rc<RefCell<bool>>,
    }

    impl<T> PanicCell<T> {
        fn new(value: T, max_drop_count: u32) -> Self {
            Self {
                value,
                max_drop_count,
                drop_counter: DropCounter::new(Rc::new(RefCell::new(0))),
                panic_enabled: Rc::new(RefCell::new(true)),
            }
        }

        fn drop_count(&self) -> u32 {
            self.drop_counter.drop_count()
        }

        fn is_panic_enabled(&self) -> bool {
            *self.panic_enabled.borrow()
        }

        fn enable_panics(&mut self) {
            *self.panic_enabled.borrow_mut() = true;
        }

        fn disable_panics(&mut self) {
            *self.panic_enabled.borrow_mut() = false;
        }
    }

    impl<T> Drop for PanicCell<T> {
        fn drop(&mut self) {
            self.drop_counter.increment();

            if self.is_panic_enabled() && (self.drop_count() > self.max_drop_count) {
                panic!(
                    "Drop threshold exceeded: {} > {} (panics {})",
                    self.drop_count(),
                    self.max_drop_count,
                    if self.is_panic_enabled() { "enabled" } else { "disabled" }
                );
            }
        }
    }

    fn new_vec<T>() -> OpaqueVecInner
    where
        T: 'static,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>)
        where
            T: core::fmt::Debug + 'static,
        {
            {
                let value_ref: &T = &*value.cast::<T>().as_ptr();

                eprintln!("Dropping value `{:?}` at memory location: `{:?}`", value_ref, value);
            }

            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let alloc = OpaqueAlloc::new(std::alloc::Global);
        let element_layout = Layout::new::<PanicCell<()>>();
        let drop_fn = Some(drop_fn::<PanicCell<()>> as unsafe fn(NonNull<u8>));

        OpaqueVecInner::new_in(alloc, element_layout, drop_fn)
    }

    #[test]
    #[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
    fn test_replace_insert_panic_calls_drop() {
        let mut panic_cell = PanicCell::new((), 2);
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let value = panic_cell.clone();
            panic!("Intentional panic to test drop counts for value: `{:?}`", value);
        }));

        assert!(result.is_err());

        panic_cell.disable_panics();

        let expected = 2;
        let result = panic_cell.drop_count();

        assert_eq!(result, expected);
    }

    #[test]
    #[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
    fn test_replace_insert_manually_drop_does_not_call_drop() {
        let mut panic_cell = PanicCell::new((), 2);
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let value = ManuallyDrop::new(panic_cell.clone());
            panic!("Intentional panic to test drop counts for value: `{:?}`", value);
        }));

        assert!(result.is_err());

        panic_cell.disable_panics();

        let expected = 0;
        let result = panic_cell.drop_count();

        assert_eq!(result, expected);
    }

    #[test]
    #[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
    fn test_replace_insert_on_panic_drop_count() {
        let mut triggering_panic_cell = PanicCell::new((), 0);
        let mut replacement_panic_cell = PanicCell::new((), 2);
        let mut vec = new_vec::<PanicCell<()>>();
        {
            // Manually implement move semantics since this a lower level operation.
            let value = ManuallyDrop::new(triggering_panic_cell.clone());
            let value_ptr = NonNull::from(&*value).cast::<u8>();
            vec.replace_insert(0, value_ptr);
        }

        assert_eq!(triggering_panic_cell.drop_count(), 0);
        assert_eq!(replacement_panic_cell.drop_count(), 0);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            // Manually implement move semantics since this a lower level operation.
            let value = ManuallyDrop::new(replacement_panic_cell.clone());
            let value_ptr = NonNull::from(&*value).cast::<u8>();
            vec.replace_insert(0, value_ptr);
        }));

        assert!(result.is_err());

        triggering_panic_cell.disable_panics();
        replacement_panic_cell.disable_panics();

        let expected = 2;
        let result = replacement_panic_cell.drop_count();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_replace_insert_on_success_drop_count() {
        let mut panic_cell = PanicCell::new((), 1);
        let mut vec = new_vec::<PanicCell<()>>();
        {
            // Manually implement move semantics since this a lower level operation.
            let value = ManuallyDrop::new(panic_cell.clone());
            let value_ptr = NonNull::from(&value).cast::<u8>();
            vec.replace_insert(0, value_ptr);
        }

        panic_cell.disable_panics();

        let expected = 0;
        let result = panic_cell.drop_count();

        assert_eq!(result, expected);
    }
}
