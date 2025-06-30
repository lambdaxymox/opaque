use crate::set::common::projected::strategy_type_projected_index_set_max_len_nonempty;
use opaque_index_map::TypedProjIndexSet;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn strategy_prop_insert_preserves_order_new_entry<T, S, A>(max_length: usize) -> impl Strategy<Value = (TypedProjIndexSet<T, S, A>, T)>
where
    T: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    strategy_type_projected_index_set_max_len_nonempty(max_length + 1)
        .prop_map(move |mut set| {
            let new_entry = set.pop().unwrap();
            (set, new_entry)
        })
}

fn prop_insert_preserves_order_new_entry<T, S, A>((entries, new_entry): (TypedProjIndexSet<T, S, A>, T)) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();

    prop_assert!(!set.contains(&new_entry));

    let values_before: Vec<T> = set.iter().cloned().collect();

    set.insert(new_entry.clone());

    let values_after: Vec<T> = set.iter().cloned().collect();

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
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    let new_entry: $value_typ = new_entry;
                    super::prop_insert_preserves_order_new_entry((entries, new_entry))?
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
    128,
    strategy_prop_insert_preserves_order_new_entry,
);
generate_props!(
    usize,
    usize,
    hash::RandomState,
    alloc::Global,
    128,
    strategy_prop_insert_preserves_order_new_entry,
);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    128,
    strategy_prop_insert_preserves_order_new_entry,
);
