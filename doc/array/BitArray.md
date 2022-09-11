# Bit-Precision Array Immediate

This type is a wrapper over the [array fundamental][0] `[T; N]` that views its
contents as a [`BitSlice`] region. As an array, it can be held directly by value
and does not require an indirection such as the `&BitSlice` reference.

## Original

[`[T; N]`](https://doc.rust-lang.org/std/primitive.array.html)

## Usage

`BitArray` is a Rust analogue of the C++ [`std::bitset<N>`] container. However,
restrictions in the Rust type system do not allow specifying exact bit lengths
in the array type. Instead, it must specify a storage array that can contain all
the bits you want.

Because `BitArray` is a plain-old-data object, its fields are public and it has
no restrictions on its interior value. You can freely access the interior
storage and move data in or out of the `BitArray` type with no cost.

As a convenience, the [`BitArr!`] type-constructor macro can produce correct
type definitions from an exact bit count and your memory-layout type parameters.
Values of that type can then be built from the [`bitarr!`] *value*-constructor
macro:

```rust
use bitvec::prelude::*;

type Example = BitArr!(for U43, in u32, Msb0);
let example: Example = bitarr!(u32, Msb0; 1; U43);

struct HasBitfield {
  inner: Example,
}

let ex2 = HasBitfield {
  inner: BitArray::new(arr![u32; 1, 2]),
};
```

Note that the actual type of the `Example` alias is
`BitArray<GenericArray<u32, U2>, Msb0, U43>`.  While this is backed by `[u32; 2]`
storage, only 43 bits of it are accessible.  The `bitarr!` macro therefore cannot
accept any number of bits `33 .. 65` while still producing a value of the correct
type.

## Type Parameters

Like the other data structures in the crate, `BitArray` takes a `T: BitStore`
parameter.

As with all `BitSlice` regions, the `O: BitOrder` parameter specifies the
ordering of bits within a single `A::Store` element.

The `N: Elts<T>` parameter is a type-level integer (from the typenum crate)
indicating the number of bits held.

## Future API Changes

Exact bit lengths are currently encoded into the `BitArray` type via type-level
integers from the typenum crate.  The const-generics system in the compiler is
a better approach, but cannot be used without type-level computation on type
integers. When this stabilizes, `bitvec` will issue a major upgrade that
replaces the `BitArray<S, O, N>` definition with `BitArray<S, O, const N: usize>`
and match the C++ `std::bitset<N>` definition.

## Large Bit-Arrays

As with ordinary arrays, large arrays can be expensive to move by value, and
should generally be preferred to have static locations such as actual `static`
bindings, a long lifetime in a low stack frame, or a heap allocation. While you
certainly can `Box<[BitArray<S, O, N>]>` directly, you may instead prefer the
[`BitBox`] or [`BitVec`] heap-allocated regions. These offer the same storage
behavior and are better optimized than `Box<BitArray>` for working with the
contained `BitSlice` region.

## Examples

```rust
use bitvec::prelude::*;

const WELL_KNOWN: BitArr!(for U16, in u8, Lsb0) = BitArray::<u8, Lsb0, U16> {
  data: arr![u8; b'b', b'v'],
  ..BitArray::ZERO
};

struct HasBitfields {
  inner: BitArr!(for U50, in u8, Lsb0),
}

impl HasBitfields {
  fn new() -> Self {
    Self {
      inner: bitarr!(u8, Lsb0; 0; U50),
    }
  }

  fn some_field(&self) -> &BitSlice<u8, Lsb0> {
    &self.inner[2 .. 52]
  }
}
```

[0]: https://doc.rust-lang.org/std/primitive.array.html
[`BitArr!`]: macro@crate::BitArr
[`BitBox`]: crate::boxed::BitBox
[`BitSlice`]: crate::slice::BitSlice
[`BitVec`]: crate::vec::BitVec
[`bitarr!`]: macro@crate::bitarr
[`std::bitset<N>`]: https://en.cppreference.com/w/cpp/utility/bitset
