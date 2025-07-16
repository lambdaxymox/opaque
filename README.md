# Opaque Collections Library

## Introduction

The **opaque** crate is a library for working with type erasable collections. This library is used for specialized 
situations where one needs the ability to manipulate heterogeneous data in a resource efficient way. ***If your use 
case only requires a regular generic collection, just stick with the standard library collections.***

Type erasable data structures can store the values of arbitrary types in a uniform way by erasing their type 
information at compile time and storing the type information in the data structure. This pattern is powerful for 
building flexible and extensible runtime systems, particularly when working with heterogeneous data or dynamic 
interfaces. The type erasable collections have an additional benefit of being highly cache coherent, since the 
type-erased data structures carry no semantic information about their contents.

Common use cases for type-erased collections include:

* **Heterogeneous Collections:** Store data of different types in a single container. This is a common scenario in e.g.
  computer graphics where on is storing handles for multiple types of resources inside another subsystem in say, a 
  game engine, or for data stored across an FFI boundary when working with say Vulkan, Metal, OpenGL, or D3D12.
* **Plugin and Serialization Systems:** Manage user-defined or dynamically loaded types whose concrete types are only 
  known at runtime.
* **Dynamic Dispatch and Message Passing:** Pass messages of various types through a single channel or queue, 
  projecting them back for processing.
* **Reflection and Scripting:** Manipulate and invoke native objects from scripting environments or reflection APIs.
* **Serialization/Deserialization:** Store and reconstruct objects across type boundaries for persistence or 
  communication.

In summary, this library is meant for situations where one requires the ability to hide type information and store data 
in a type erased form, but do so in a cache-coherent and memory efficient way.

## Getting Started

To use this library in your project, add the **opaque** crate as a dependency in your `Cargo.toml` file

```toml
[dependencies]
opaque = "0.2.0"
```

This library has a `nightly` feature to unlock using custom memory allocators for all the collections in the 
library. To use `nightly` add

```toml
[dependencies.opaque]
version = "0.2.0"
features = ["nightly"]
```

or 

```toml
[dependencies]
opaque = { version = "0.2.0", features = ["nightly"] }
```

to your `Cargo.toml` file. Optionally, you can add the crate declaration

```rust
extern crate opaque;
```

to your `lib.rs` or `main.rs` file, but this is not strictly necessary.

## Testing

To run the tests for the library, run

```text
cargo test --workspace
```

on stable Rust. Run

```text
cargo +nightly test --workspace --features "nightly" 
```

to run the tests with nightly features enabled.

## Features

This library has numerous features:

* Every type-projected data structure can be type-erased. Every type-erased data structure can be type-projected.
  Either direction can always be done no matter what as long as the user supplies the correct type information.
* Support for type-erasable memory allocators.
* Support for type-erasable hashing functions and hash builders.
* Support for type-erasable vectors.
* Support for type-erasable hash maps and hash sets with compact storage.
* Support for custom memory allocators on all collections on both Rust nightly and Rust stable, where stable support 
  comes through the `opaque_allocator_api` polyfill subcrate. This is a temporary feature until the `allocator_api` 
  feature stabilizes. **This is a future breaking change. When `allocator_api` stabilizes, this feature will be 
  removed**.
* Extensive documentation including formal axiomatic semantics for the trickier to understand mutation operations.
* The hash map, hash set, and vector APIs are designed to be almost identical to the standard library collection
  counterparts in terms of both the feature set, and the method names, to make them more familiar to work with.
* Working with type-erased data structures is always type safe. A runtime type error, i.e. the user provides the wrong
  type information is considered an unrecoverable error, and will result in a panic. 

## Type Erasure And Type Projection

**Type erasure** is the process of hiding or removing concrete type information from a value or data structure, while 
preserving its behavior through a common interface. In Rust, type erasure is commonly achieved using trait objects 
like `Box<dyn Trait>`, which allow different types implementing the same trait to be stored together.

**Type projection** is the process of recovering or “projecting” a value stored in a type-erased form back to its 
original concrete type. This typically involves downcasting or checking type information at runtime, and is only 
successful if the stored value matches the requested type.

## Performance Tradeoffs Compared To Other Approaches

One could also get type erasure using something like `Box<dyn Any>`, but every single boxed instance creates another 
allocation on the heap, whereas the type erasable containers can just store the data directly inside the container, 
preventing the need to allocate separately for each element in the collection. This improves performance by reducing 
the number of calls to the memory allocator, improves cache coherence by storing the elements compactly in the case of 
a linear storage container, and reducing the amount of dynamic dispatch needed to operate on the elements of the 
collection.

There are performance tradeoffs compared to working with standard generic collections too. In order to make type 
erasure work with memory efficient containers, the containers need to store copies of their runtime type information
inside the container itself. This adds a small increase in the memory footprint of the collection, though this is 
minor. In order to maintain the integrity of the type-erased collection, any mutation of the collection must do a 
type-safe type projection to unlock the underlying element types to work with them. Type projection must do a runtime
type check to ensure that the supplied generic types runtime type information exactly matches the runtime type
information stored in the collection. This means that operating on a type-erased collection will not be as fast as 
either its type-projected or an equivalent generic collection.

## Prior Art

The type-projected and type-erased vector data structures are largely based on the `Vec` data structure from the Rust 
[standard library](https://github.com/rust-lang/rust). Attribution is provided in the `opaque_vec` subcrate. The 
type-projected and type-erased hash map and hash set data structures are largely based on the 
[`indexmap`](https://github.com/indexmap-rs/indexmap) crate. Attribution is provided in the `opaque_index_map` subcrate.
