#![warn(missing_docs, rust_2018_idioms)]
#![cfg_attr(not(feature = "alloc"), no_std)]

/*!
# Description

This crate provides array-based set and map data structures. These structures
have a fixed capacity but keep track of how many elements the user has inserted
and removed.

- [`ArrayMap`](array_map/struct.ArrayMap.html) is an array-backed map
- [`ArraySet`](array_set/struct.ArraySet.html) is an array-backed set

If the `alloc` feature is enabled (which it is by default), this crate also
provides variants of these stack-based structures that automatically move to the
heap if they grow beyond their array's capacity.

- [`TinyMap`](tiny_map/enum.TinyMap.html) is an auto-allocating map
- [`TinySet`](tiny_set/enum.TinySet.html) is an auto-allocating set

# Array Types

The underlying arrays for ArrayMap and ArraySet are of a special kind where the
items are wrapped in an [`Inner`](type.Inner.html). The reasons for this are related
to performance. Suffice it to say, without this requirement, many more ArrayMap and
ArraySet functions would require that their items implement `Default` and would be
less efficient.

- Arrays for ArraySets must implement [`Array`](trait.Array.html).
    - Implemented for `[Inner<T>; N]` for all `T` and for `N` up to 50
- Arrays for ArrayMap must implement [`MapArray`](trait.MapArray.html).
    - Implemented for `[Inner<(K, V)>; N]` for all `K` and `V` and for `N` up to 50

# Macros

Until [const generics](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md)
are available on stable, the type signatures for types from this crate will be quite ugly.

Creating a new ArrayMap of `i32`s to `&str`s with a capacity of 10 looks like this:
```
# use tinymap::*;
let map = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
```

Because of this verbosity, this crate provides a macro for each container type to easily
create one with the desired type. For example, with the `arraymap!` macro, the above code becomes:
```
# use tinymap::*;
let map = arraymap!(i32 => &str; 10);
```

# Serialization

Serde serialization and deserialization for the data structures in this crate can be enabled
with the `serde` feature.
*/

pub mod array_map;
pub mod array_set;
#[cfg(feature = "serde")]
mod serialize;
#[cfg(test)]
#[cfg(feature = "alloc")]
mod test;
#[cfg(feature = "alloc")]
pub mod tiny_map;
#[cfg(feature = "alloc")]
pub mod tiny_set;

pub use array_map::ArrayMap;
pub use array_set::ArraySet;
#[cfg(feature = "alloc")]
pub use tiny_map::TinyMap;
#[cfg(feature = "alloc")]
pub use tiny_set::TinySet;

use core::mem::MaybeUninit;

/**
Create a new ArrayMap with the specified parameters

# Expansion

```ignore
arraymap!( KEY_TYPE => VALUE_TYPE; CAPACITY ) -> tinymap::ArrayMap::<[tinymap::Inner<(KEY_TYPE, VALUE_TYPE)>; CAPACITY]>::new()
```

# Example

```
use tinymap::arraymap;

let mut map = arraymap!(i32 => &str; 10);
map.insert(1, "a");
```
*/
#[macro_export]
macro_rules! arraymap {
    ($k:ty => $v:ty; $n:expr) => {
        tinymap::ArrayMap::<[tinymap::Inner<($k, $v)>; $n]>::new()
    };
}

/**
Create a new ArraySet with the specified parameters

# Expansion

```ignore
arrayset!( VALUE_TYPE; CAPACITY ) -> tinymap::ArraySet::<[tinymap::Inner<VALUE_TYPE>; CAPACITY]>::new()
```

# Example

```
use tinymap::arrayset;

let mut set = arrayset!(i32; 10);
set.insert(1);
```
*/
#[macro_export]
macro_rules! arrayset {
    ($v:ty; $n:expr) => {
        tinymap::ArraySet::<[tinymap::Inner<$v>; $n]>::new()
    };
}

/**
Create a new TinyMap with the specified parameters

# Expansion

```ignore
tinymap!( KEY_TYPE => VALUE_TYPE; CAPACITY ) -> tinymap::TinyMap::<[tinymap::Inner<(KEY_TYPE, VALUE_TYPE)>; CAPACITY]>::new()
```

# Example

```
use tinymap::tinymap;

let mut map = tinymap!(i32 => &str; 10);
map.insert(1, "a");
```
*/
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! tinymap {
    ($k:ty => $v:ty; $n:expr) => {
        tinymap::TinyMap::<[tinymap::Inner<($k, $v)>; $n]>::new()
    };
}

/**
Create a new TinySet with the specified parameters

# Expansion

```ignore
tinyset!( VALUE_TYPE; CAPACITY ) -> tinymap::TinySet::<[tinymap::Inner<VALUE_TYPE>; CAPACITY]>::new()
```

# Example

```
use tinymap::tinyset;

let mut set = tinyset!(i32; 10);
set.insert(1);
```
*/
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! tinyset {
    ($v:ty; $n:expr) => {
        tinymap::TinySet::<[tinymap::Inner<$v>; $n]>::new()
    };
}

/// An entry in an array
pub type Inner<T> = MaybeUninit<T>;

/// Behavior for an array
pub trait Array {
    /// The Item type
    type Item;
    /// The array's capacity
    const CAPACITY: usize;
    /// Get a slice into the array
    fn as_slice(&self) -> &[Inner<Self::Item>];
    /// Get a mutable slice into the array
    fn as_mut_slice(&mut self) -> &mut [Inner<Self::Item>];
    /// Turn the array into a boxed slice
    #[cfg(feature = "alloc")]
    fn into_boxed_slice(self) -> Box<[Inner<Self::Item>]>;
}

/// Behavior for a map array
pub trait MapArray {
    /// The Key type
    type Key;
    /// The Value type
    type Value;
    /// The array's capacity
    const CAPACITY: usize;
    /// Get a slice into the array
    fn as_slice(&self) -> &[Inner<(Self::Key, Self::Value)>];
    /// Get a mutable slice into the array
    fn as_mut_slice(&mut self) -> &mut [Inner<(Self::Key, Self::Value)>];
    /// Turn the array into a boxed slice
    #[cfg(feature = "alloc")]
    fn into_boxed_slice(self) -> Box<[Inner<(Self::Key, Self::Value)>]>;
}

macro_rules! impl_array {
    ($($n:literal),*) => {
        $(
            impl<T> Array for [Inner<T>; $n]  {
                type Item = T;
                const CAPACITY: usize = $n;
                fn as_slice(&self) -> &[Inner<Self::Item>] {
                    self
                }
                fn as_mut_slice(&mut self) -> &mut [Inner<Self::Item>] {
                    self
                }
                #[cfg(feature = "alloc")]
                fn into_boxed_slice(self) -> Box<[Inner<Self::Item>]> {
                    Box::new(self)
                }
            }
            impl<K, V> MapArray for [Inner<(K, V)>; $n]  {
                type Key = K;
                type Value = V;
                const CAPACITY: usize = $n;
                fn as_slice(&self) -> &[Inner<(Self::Key, Self::Value)>] {
                    self
                }
                fn as_mut_slice(&mut self) -> &mut [Inner<(Self::Key, Self::Value)>] {
                    self
                }
                #[cfg(feature = "alloc")]
                fn into_boxed_slice(self) -> Box<[Inner<(Self::Key, Self::Value)>]> {
                    Box::new(self)
                }
            }
        )*
    };
}

impl_array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50
);
