#![warn(missing_docs, rust_2018_idioms)]

/*!

*/

pub mod array_map;

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
}

impl<K, V> MapEntry for (K, V) {
    type Key = K;
    type Value = V;
    fn new(k: Self::Key, v: Self::Value) -> Self {
        (k, v)
    }
    fn into_pair(self) -> (Self::Key, Self::Value) {
        self
    }
    fn key(&self) -> &Self::Key {
        &self.0
    }
    fn value(&self) -> &Self::Value {
        &self.1
    }
    fn value_mut(&mut self) -> &mut Self::Value {
        &mut self.1
    }
}

/// Dehavior for an array
pub trait Array {
    /// The Item type
    type Item;
    /// The array's capacity
    const CAPACITY: usize;
    /// Get a slice into the array
    fn as_slice(&self) -> &[Self::Item];
    /// Get a mutable slice into the array
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
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
            }
        )*
    };
}

impl_array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50
);
