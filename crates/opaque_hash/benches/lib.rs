#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![no_std]
extern crate std;

mod bench_opaque_hasher;
mod bench_typed_proj_hasher;

use criterion::criterion_main;

criterion_main!(
    bench_opaque_hasher::bench_opaque_hasher,
    bench_typed_proj_hasher::bench_typed_proj_hasher,
);
