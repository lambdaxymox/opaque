use opaque_index_map::map::TypedProjIndexMap;

use std::alloc;
use std::cell::RefCell;
use std::panic;
use std::panic::AssertUnwindSafe;
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

#[test]
#[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
fn test_truncate_on_panic_drop_count1() {
    let mut triggering_panic_cell = PanicCell::new((), 0);
    let mut map = TypedProjIndexMap::new();

    map.insert(0, triggering_panic_cell.clone());

    assert_eq!(triggering_panic_cell.drop_count(), 0);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        map.clear();
    }));

    assert!(result.is_err());

    triggering_panic_cell.disable_panics();

    assert_eq!(triggering_panic_cell.drop_count(), 2);
}

#[test]
#[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
fn test_truncate_on_panic_drop_count2() {
    let mut triggering_panic_cell = PanicCell::new((), 0);
    let mut panic_cell = PanicCell::new((), 2);
    let mut map = TypedProjIndexMap::new();

    map.insert(0, triggering_panic_cell.clone());
    map.insert(1, panic_cell.clone());

    assert_eq!(triggering_panic_cell.drop_count(), 0);
    assert_eq!(panic_cell.drop_count(), 0);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        map.clear();
    }));

    assert!(result.is_err());

    triggering_panic_cell.disable_panics();
    panic_cell.disable_panics();

    assert_eq!(triggering_panic_cell.drop_count(), 2);
    assert_eq!(panic_cell.drop_count(), 2);
}

#[test]
#[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
fn test_truncate_on_panic_drop_count3() {
    let mut triggering_panic_cell = PanicCell::new((), 0);
    let mut panic_cell = PanicCell::new((), 4);
    let mut map = TypedProjIndexMap::new();

    map.insert(0, panic_cell.clone());
    map.insert(1, triggering_panic_cell.clone());
    map.insert(2, panic_cell.clone());

    assert_eq!(triggering_panic_cell.drop_count(), 0);
    assert_eq!(panic_cell.drop_count(), 0);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        map.clear();
    }));

    assert!(result.is_err());

    triggering_panic_cell.disable_panics();
    panic_cell.disable_panics();

    assert_eq!(triggering_panic_cell.drop_count(), 2);
    assert_eq!(panic_cell.drop_count(), 4);
}

#[test]
fn test_truncate_on_success_drop_count() {
    let mut panic_cell = PanicCell::new((), 2);
    let mut map = TypedProjIndexMap::new();

    map.insert(0, panic_cell.clone());

    assert_eq!(panic_cell.drop_count(), 0);

    map.clear();
    panic_cell.disable_panics();

    assert_eq!(panic_cell.drop_count(), 2);
}
