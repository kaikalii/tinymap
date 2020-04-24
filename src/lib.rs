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
heap if the grow beyond their array's capacity.

- [`TinyMap`](tiny_map/struct.TinyMap.html) is an auto-allocating map
- [`TinySet`](tiny_set/struct.TinySet.html) is an auto-allocating set
*/

pub mod array_map;
pub mod array_set;
#[cfg(test)]
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
arraymap!( KEY_TYPE => VALUE_TYPE; CAPACITY ) -> tinymap::ArrayMap::<[Entry<(KEY_TYPE, VALUE_TYPE)>; CAPACITY]>::new()
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
        tinymap::ArrayMap::<[($k, $v); $n]>::new()
    };
}

/**
Create a new ArraySet with the specified parameters

# Expansion

```ignore
arrayset!( VALUE_TYPE; CAPACITY ) -> tinymap::ArraySet::<[VALUE_TYPE; CAPACITY]>::new()
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
        tinymap::ArraySet::<[$v; $n]>::new()
    };
}

/**
Create a new ArrayMap with the specified parameters

# Expansion

```ignore
tinymap!( KEY_TYPE => VALUE_TYPE; CAPACITY ) -> tinymap::TinyMap::<[(KEY_TYPE, VALUE_TYPE); CAPACITY]>::new()
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
        tinymap::TinyMap::<[($k, $v); $n]>::new()
    };
}

/**
Create a new TinySet with the specified parameters

# Expansion

```ignore
tinyset!( VALUE_TYPE; CAPACITY ) -> tinymap::TinySet::<[VALUE_TYPE; CAPACITY]>::new()
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
        tinymap::TinySet::<[$v; $n]>::new()
    };
}

/// An entry in an array
pub type Entry<T> = MaybeUninit<T>;

/// Behavior for an entry in a map
pub trait MapEntry {
    /// The key type
    type Key;
    /// The value type
    type Value;
    /// Create a new entry
    fn new(k: Self::Key, v: Self::Value) -> Self;
    /// Turn the entry into a key-value pair
    fn into_pair(self) -> (Self::Key, Self::Value);
    /// Get a reference to the key
    fn key(&self) -> &Self::Key;
    /// Get a reference to the value
    fn value(&self) -> &Self::Value;
    /// Get a mutable reference to the value
    fn value_mut(&mut self) -> &mut Self::Value;
    /// Get mutable references to the key and value
    fn as_mut_pair(&mut self) -> (&mut Self::Key, &mut Self::Value);
    /// Drop the entry
    fn drop(&mut self);
}

impl<K, V> MapEntry for Entry<(K, V)> {
    type Key = K;
    type Value = V;
    fn new(k: Self::Key, v: Self::Value) -> Self {
        Entry::new((k, v))
    }
    fn into_pair(self) -> (Self::Key, Self::Value) {
        unsafe { self.assume_init() }
    }
    fn key(&self) -> &Self::Key {
        &unsafe { self.as_ptr().as_ref() }.unwrap().0
    }
    fn value(&self) -> &Self::Value {
        &unsafe { self.as_ptr().as_ref() }.unwrap().1
    }
    fn value_mut(&mut self) -> &mut Self::Value {
        &mut unsafe { self.as_mut_ptr().as_mut() }.unwrap().1
    }
    fn as_mut_pair(&mut self) -> (&mut Self::Key, &mut Self::Value) {
        let pair = unsafe { self.as_mut_ptr().as_mut() }.unwrap();
        (&mut pair.0, &mut pair.1)
    }
    fn drop(&mut self) {
        unsafe { self.as_mut_ptr().drop_in_place() }
    }
}

/// Behavior for an array
pub trait Array {
    /// The Item type
    type Item;
    /// The array's capacity
    const CAPACITY: usize;
    ///
    /// Get a slice into the array
    fn as_slice(&self) -> &[Self::Item];
    /// Get a mutable slice into the array
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
    /// Turn the array into a boxed slice
    #[cfg(feature = "alloc")]
    fn into_boxed_slice(self) -> Box<[Self::Item]>;
}

macro_rules! impl_array {
    ($($n:literal),*) => {
        $(
            impl<T> Array for [T; $n]  {
                type Item = T;
                const CAPACITY: usize = $n;
                fn as_slice(&self) -> &[Self::Item] {
                    self
                }
                fn as_mut_slice(&mut self) -> &mut [Self::Item] {
                    self
                }
                #[cfg(feature = "alloc")]
                fn into_boxed_slice(self) -> Box<[Self::Item]> {
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
