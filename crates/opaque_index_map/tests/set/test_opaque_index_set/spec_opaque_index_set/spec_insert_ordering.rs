use crate::set::common::erased::{
    SingleBoundedValue,
    strategy_type_erased_index_set_max_len_nonempty,
};
use opaque_index_map::OpaqueIndexSet;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn strategy_prop_insert_preserves_order_new_entry<T, S, A>(max_length: usize) -> impl Strategy<Value = (OpaqueIndexSet, T)>
where
    T: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    strategy_type_erased_index_set_max_len_nonempty::<T, S, A>(max_length + 1)
        .prop_map(move |mut set| {
            let new_entry = set.pop::<T, S, A>().unwrap();
            (set, new_entry)
        })
}

fn prop_insert_preserves_order_new_entry<T, S, A>((entries, new_entry): (OpaqueIndexSet, T)) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone::<T, S, A>();

    prop_assert!(!set.contains::<_, T, S, A>(&new_entry));

    let values_before: Vec<T> = set.iter::<T, S, A>().cloned().collect();

    set.insert::<T, S, A>(new_entry.clone());

    let values_after: Vec<T> = set.iter::<T, S, A>().cloned().collect();

    let expected = {
        let mut _vec = values_before.clone();
        _vec.push(new_entry.clone());
        _vec
    };
    let result = values_after;

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    (
        $module_name:ident,
        $value_typ:ty,
        $build_hasher_typ:ty,
        $alloc_typ:ty,
        $max_length:expr,
        $set_gen:ident,
    ) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::hash;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_insert_preserves_order_new_entry((entries, new_entry) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    let new_entry: $value_typ = new_entry;
                    super::prop_insert_preserves_order_new_entry::<$value_typ, $build_hasher_typ, $alloc_typ>((entries, new_entry))?
                }
            }
        }
    };
}

generate_props!(
    u64,
    u64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_insert_preserves_order_new_entry,
);
generate_props!(
    usize,
    usize,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_insert_preserves_order_new_entry,
);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_insert_preserves_order_new_entry,
);
