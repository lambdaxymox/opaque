#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![no_std]
extern crate std;

mod bench_type_erased_hasher;
mod bench_type_projected_hasher;

use criterion::criterion_main;

criterion_main!(
    bench_type_erased_hasher::bench_type_erased_hasher,
    bench_type_projected_hasher::bench_type_projected_hasher,
);
