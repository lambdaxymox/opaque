use core::hash;

use opaque_vec::OpaqueVec;

pub fn key_value_pairs<'a, 'b, K, V, I, J>(keys: I, values: J) -> OpaqueVec
where
    K: Clone + Eq + hash::Hash + 'static,
    V: Clone + Eq + 'static,
    I: Iterator<Item = &'a K>,
    J: Iterator<Item = &'b V>,
{
    OpaqueVec::from_iter(keys.cloned().zip(values.cloned()))
}
