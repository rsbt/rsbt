#![cfg_attr(not(feature = "std"), no_std)]

/*!
# rsbt-memory-hub description

Fixed-size memory arena allocator for constrained environments.
Designed for Raspberry Pi (~128MB) and similar devices.

## Features

- Bump allocator with fixed-size buffer
- Zero-copy integration with bencode parsing
- Support for `no_std` with `alloc`

## Usage

```rust
use rsbt_memory_hub::MemoryArena;

let mut arena = MemoryArena::new(1024 * 1024); // 1MB arena
let string = arena.alloc_str("hello");
```

*/

pub mod arena;

pub use arena::MemoryArena;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn arena_alloc() {
        let mut arena = MemoryArena::new(1024);
        let _value = arena.alloc(42i32);
        let _string = arena.alloc_str("hello");
    }

    #[test]
    fn arena_vec() {
        let mut arena = MemoryArena::new(1024);
        let mut vec = arena.alloc_vec::<i32>();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
    }
}
