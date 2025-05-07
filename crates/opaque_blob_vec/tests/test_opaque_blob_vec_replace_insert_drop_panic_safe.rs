#![feature(allocator_api)]
use opaque_alloc::OpaqueAlloc;
use opaque_blob_vec::OpaqueBlobVec;

use std::alloc::Layout;
use std::mem::ManuallyDrop;
use std::panic::{
    self,
    AssertUnwindSafe,
};

use core::any;
use core::fmt;

use std::cell::RefCell;
use std::ptr::NonNull;
use std::rc::Rc;

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

fn new_vec<T>() -> OpaqueBlobVec
where
    T: any::Any + fmt::Debug,
{
    unsafe fn drop_fn<T>(value: NonNull<u8>)
    where
        T: any::Any + fmt::Debug,
    {
        {
            let value_ref: &T = &*value.cast::<T>().as_ptr();

            eprintln!("Dropping value `{:?}` at memory location: `{:?}`", value_ref, value);
        }

        let to_drop = value.as_ptr() as *mut T;

        core::ptr::drop_in_place(to_drop)
    }

    let alloc = OpaqueAlloc::new(std::alloc::Global);
    let element_layout = Layout::new::<T>();
    let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));

    OpaqueBlobVec::new_in(alloc, element_layout, drop_fn)
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
