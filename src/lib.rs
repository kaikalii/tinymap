#![warn(missing_docs, rust_2018_idioms)]

/*!

*/

use std::{borrow::Borrow, mem::swap, ops::Index};

/**
An array-backed, map-like data structure.

ArrayMap wraps an array of key-value pairs and supports operation similar to a BTreeMap or HashMap.
It has a fixed capacity, but it keeps track of how many pairs have been inserted and removed.

Because this crate uses no unsafe code, key and value types must both implement Default.

# Efficiency

In general...
*/
#[derive(Clone, Copy, Default)]
pub struct ArrayMap<A> {
    array: A,
    len: usize,
}

impl<A> ArrayMap<A>
where
    A: Default,
{
    /**
    Creates a new empty ArrayMap

    # Example
    ```
    use tinymap::ArrayMap;

    let mut map = ArrayMap::<[(i32, &str); 10]>::new();

    // entries can now be inserted into the empty map
    map.insert(1, "a");
    ```
    */
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A> ArrayMap<A> {
    /**
    Returns the number of elements in the map

    # Example

    ```
    use tinymap::ArrayMap;

    let mut a = ArrayMap::<[(i32, &str); 10]>::new();
    assert_eq!(a.len(), 0);
    a.insert(1, "a");
    assert_eq!(a.len(), 1);
    ```
    */
    pub const fn len(&self) -> usize {
        self.len
    }
    /**
    Returns `true` if the map contains no elements

    # Example

    ```
    use tinymap::ArrayMap;

    let mut a = ArrayMap::<[(i32, &str); 10]>::new();
    assert!(a.is_empty());
    a.insert(1, "a");
    assert!(!a.is_empty());
    ```
    */
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<A, K, V> ArrayMap<A>
where
    A: Array,
    A::Item: MapEntry<Key = K, Value = V>,
    K: Ord,
{
    /**
    Inserts a key-value pair into the map

    # Example

    ```
    use tinymap::ArrayMap;

    let mut map = ArrayMap::<[(i32, &str); 10]>::new();
    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.is_empty(), false);

    map.insert(37, "b");
    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");
    ```
    */
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.find(&key) {
            Ok(i) => {
                let mut entry = A::Item::new(key, value);
                swap(&mut entry, &mut self.array.as_mut_slice()[i]);
                let (_, value) = entry.into_pair();
                Some(value)
            }
            Err(i) => {
                let slice = self.array.as_mut_slice();
                for j in ((i + 1)..self.len).rev() {
                    slice.swap(j - 1, j);
                }
                let mut entry = A::Item::new(key, value);
                swap(&mut entry, &mut slice[i]);
                self.len += 1;
                None
            }
        }
    }
}

impl<A, K, V> ArrayMap<A>
where
    A: Array,
    A::Item: MapEntry<Key = K, Value = V>,
{
    fn find<Q>(&self, key: &Q) -> Result<usize, usize>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.array.as_slice()[0..self.len]
            .binary_search_by_key(&key.borrow(), |entry| entry.key().borrow())
    }
    /**
    Returns true if the map contains a value for the specified key

    # Example

    ```
    use tinymap::ArrayMap;

    let mut map = ArrayMap::<[(i32, &str); 10]>::new();
    map.insert(1, "a");
    assert_eq!(map.contains_key(&1), true);
    assert_eq!(map.contains_key(&2), false);
    ```
    */
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.find(key).is_ok()
    }
    /**
    Returns a reference to the value corresponding to the key

    # Example

    ```
    use tinymap::ArrayMap;

    let mut map = ArrayMap::<[(i32, &str); 10]>::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), None);
    ```
    */
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        if let Ok(i) = self.find(key) {
            Some(self.array.as_slice()[i].value())
        } else {
            None
        }
    }
    /**
    Returns a mutable reference to the value corresponding to the key

    # Example

    ```
    use tinymap::ArrayMap;

    let mut map = ArrayMap::<[(i32, &str); 10]>::new();
    map.insert(1, "a");
    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }
    assert_eq!(map[&1], "b");
    ```
    */
    pub fn get_mut<'a, Q>(&'a mut self, key: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q> + 'a,
        Q: Ord,
    {
        if let Ok(i) = self.find(key) {
            Some(self.array.as_mut_slice()[i].value_mut())
        } else {
            None
        }
    }
}

impl<A, K, V> ArrayMap<A>
where
    A: Array,
    A::Item: MapEntry<Key = K, Value = V>,
    K: Ord + Default,
    V: Default,
{
    /**
    Removes a key from the map, returning the value at the key if the key was previously in the map

    # Example

    ```
     use tinymap::ArrayMap;

    let mut map = ArrayMap::<[(i32, &str); 10]>::new();
    map.insert(1, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);
    ```
    */
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        if let Ok(i) = self.find(key) {
            let slice = self.array.as_mut_slice();
            for j in (i + 1)..self.len {
                slice.swap(j - 1, j);
            }
            let mut entry = A::Item::new(K::default(), V::default());
            swap(&mut entry, &mut slice[i]);
            let (_, value) = entry.into_pair();
            Some(value)
        } else {
            None
        }
    }
}

impl<A, K, V, Q> Index<&Q> for ArrayMap<A>
where
    A: Array,
    A::Item: MapEntry<Key = K, Value = V>,
    K: Borrow<Q>,
    Q: Ord,
{
    type Output = V;
    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key)
            .unwrap_or_else(|| panic!("No entry found for key"))
    }
}

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
