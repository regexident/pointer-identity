# Pointer Identity

This crate gives you a wrapper type that you can use to change how your values are compared
and hashed.

## Identity

Rust has some traits that operate on values:

- `Ord` and `PartialOrd` check the ordering of values,
- `Eq` and `PartialEq` check equality of values,
- `Hash` computes a hash sum of values.

These work on the *values* of the data. For example, two strings `"Hello"` and `"Hello"` will
compute as being identical, even if they exist in different places in memory:

```rust
let left = String::from("Hello");
let right = String::from("Hello");
assert_eq!(left, right);
```


When using smart pointers such as `Arc`, `Rc` and `Box` in
Rust, these will forward the implementation to the underlying types. Generally speaking, this
makes sense and is the intended behavior.

However, in some cases this might not be what you need. For example, in some cases where you
have a cache of reference-counted values, you may want it to determine if two values are the
same just by looking at their pointer address rather than inspecting the value.

This crate lets you achieve that by offering a wrapper type, `PointerIdentity`, that you can
use to wrap any value which implements `Pointer` to get it to use the pointer address as
identity for comparisons and hashing rather than the value of the data it is holding.

## Similar Work

- [mbrubeck/by_address](https://github.com/mbrubeck/by_address/)

## License

MIT.
