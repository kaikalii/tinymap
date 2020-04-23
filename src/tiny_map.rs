//! A map that starts on the stack but can automatically move to the heap

use core::{borrow::Borrow, fmt, iter::FromIterator, mem::swap, ops::Index};
use std::collections::BTreeMap;

use crate::{Array, ArrayMap, MapEntry};

/**
A map that starts on the stack but can automatically move to the heap
*/
pub enum TinyMap<A>
where
    A: Array,
    A::Item: MapEntry,
{
    /// An map with items on the stack
    Stack(ArrayMap<A>),
    /// A map with items in the heap
    Heap(BTreeMap<<A::Item as MapEntry>::Key, <A::Item as MapEntry>::Value>),
}

impl<A> TinyMap<A>
where
    A: Array + Default,
    A::Item: MapEntry,
{
    /**
    Creates a new empty TinyMap

    # Example
    ```
    use tinymap::TinyMap;

    let mut map = TinyMap::<[(i32, &str); 10]>::new();

    // entries can now be inserted into the empty map
    map.insert(1, "a");
    ```
    */
    pub fn new() -> Self {
        Self::default()
    }
    /**
    Clears the map, removing all elements

    # Example
    ```
    use tinymap::TinyMap;

    let mut a = TinyMap::<[(i32, &str); 10]>::new();
    a.insert(1, "a");
    a.clear();
    assert!(a.is_empty());
    ```
    */
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl<A> Default for TinyMap<A>
where
    A: Array + Default,
    A::Item: MapEntry,
{
    fn default() -> Self {
        TinyMap::Stack(ArrayMap::default())
    }
}

impl<A> TinyMap<A>
where
    A: Array,
    A::Item: MapEntry,
{
    /**
    Returns the number of elements in the map

    # Example

    ```
    use tinymap::TinyMap;

    let mut a = TinyMap::<[(i32, &str); 10]>::new();
    assert_eq!(a.len(), 0);
    a.insert(1, "a");
    assert_eq!(a.len(), 1);
    ```
    */
    pub fn len(&self) -> usize {
        match self {
            TinyMap::Stack(map) => map.len(),
            TinyMap::Heap(map) => map.len(),
        }
    }
    /**
    Returns `true` if the map contains no elements

    # Example

    ```
    use tinymap::TinyMap;

    let mut a = TinyMap::<[(i32, &str); 10]>::new();
    assert!(a.is_empty());
    a.insert(1, "a");
    assert!(!a.is_empty());
    ```
    */
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<A> TinyMap<A>
where
    A: Array,
    A::Item: MapEntry,
{
    /**
    Returns the maximum number of elements the map can contain on the stack

    # Example

    ```
    use tinymap::TinyMap;

    let mut a = TinyMap::<[(i32, &str); 10]>::new();
    assert_eq!(10, a.capacity());
    ```
    */
    pub fn capacity(&self) -> usize {
        A::CAPACITY
    }
    /**
    Gets an iterator over the entries of the map, sorted by key

    # Example

    ```
    use tinymap::TinyMap;

    let mut map = TinyMap::<[(i32, &str); 10]>::new();
    map.insert(3, "c");
    map.insert(2, "b");
    map.insert(1, "a");

    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }

    let (first_key, first_value) = map.iter().next().unwrap();
    assert_eq!((*first_key, *first_value), (1, "a"));
    ```
    */
    pub fn iter(&self) -> Iter<'_, A> {
        match self {
            TinyMap::Stack(map) => Iter::Stack(map.iter()),
            TinyMap::Heap(map) => Iter::Heap(map.iter()),
        }
    }
}

impl<A, K, V> TinyMap<A>
where
    A: Array + Default,
    A::Item: MapEntry<Key = K, Value = V>,
    K: Ord,
{
    /**
    Inserts a key-value pair into the map

    If the map did not have this key present, None is returned.

    If the map did have this key present, the key and value are updated, and the old value is returned.

    # Panics

    Panics if insertion would cause the map to excede its capacity.

    # Example

    ```
    use tinymap::TinyMap;

    let mut map = TinyMap::<[(i32, &str); 10]>::new();
    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.is_empty(), false);

    map.insert(37, "b");
    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");
    ```
    */
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self {
            TinyMap::Stack(map) => match map.try_insert(key, value) {
                Ok(res) => res,
                Err((k, v)) => {
                    let mut replacement_map = ArrayMap::default();
                    swap(&mut replacement_map, map);
                    let mut btree_map = BTreeMap::new();
                    for (k, v) in replacement_map.into_iter().map(MapEntry::into_pair) {
                        btree_map.insert(k, v);
                    }
                    let res = btree_map.insert(k, v);
                    *self = TinyMap::Heap(btree_map);
                    res
                }
            },
            TinyMap::Heap(map) => map.insert(key, value),
        }
    }
}

impl<A> TinyMap<A>
where
    A: Array,
    A::Item: MapEntry,
    <A::Item as MapEntry>::Key: Ord,
{
    /**
    Returns a reference to the value corresponding to the key

    # Example

    ```
    use tinymap::TinyMap;

    let mut map = TinyMap::<[(i32, &str); 10]>::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), None);
    ```
    */
    pub fn get<Q>(&self, key: &Q) -> Option<&<A::Item as MapEntry>::Value>
    where
        <A::Item as MapEntry>::Key: Borrow<Q>,
        Q: Ord,
    {
        match self {
            TinyMap::Stack(map) => map.get(key),
            TinyMap::Heap(map) => map.get(key),
        }
    }
}

impl<A, K, V> TinyMap<A>
where
    A: Array,
    A::Item: MapEntry<Key = K, Value = V>,
    K: Ord,
{
    /**
    Returns true if the map contains a value for the specified key

    # Example

    ```
    use tinymap::TinyMap;

    let mut map = TinyMap::<[(i32, &str); 10]>::new();
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
        match self {
            TinyMap::Stack(map) => map.contains_key(key),
            TinyMap::Heap(map) => map.contains_key(key),
        }
    }
    /**
    Returns a mutable reference to the value corresponding to the key

    # Example

    ```
    use tinymap::TinyMap;

    let mut map = TinyMap::<[(i32, &str); 10]>::new();
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
        match self {
            TinyMap::Stack(map) => map.get_mut(key),
            TinyMap::Heap(map) => map.get_mut(key),
        }
    }
    /**
    Gets a mutable iterator over the entries of the map, sorted by key

    # Example

    ```
    use tinymap::TinyMap;

    let mut map = TinyMap::<[(&str, i32); 10]>::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // add 10 to the value if the key isn't "a"
    for (key, value) in map.iter_mut() {
        if key != &"a" {
            *value += 10;
        }
    }
    ```
    */
    pub fn iter_mut(&mut self) -> IterMut<'_, A> {
        match self {
            TinyMap::Stack(map) => IterMut::Stack(map.iter_mut()),
            TinyMap::Heap(map) => IterMut::Heap(map.iter_mut()),
        }
    }
    /**
    Gets an iterator over the keys of the map, sorted

    # Example

    ```
    use tinymap::TinyMap;

    let mut a = TinyMap::<[(i32, &str); 10]>::new();
    a.insert(2, "b");
    a.insert(1, "a");

    let keys: Vec<_> = a.keys().cloned().collect();
    assert_eq!(keys, [1, 2]);
    ```
    */
    pub fn keys(&self) -> Keys<'_, A> {
        match self {
            TinyMap::Stack(map) => Keys::Stack(map.keys()),
            TinyMap::Heap(map) => Keys::Heap(map.keys()),
        }
    }
    /**
    Gets an iterator over the values of the map, sorted

    # Example

    ```
    use tinymap::TinyMap;

    let mut a = TinyMap::<[(i32, &str); 10]>::new();
    a.insert(1, "hello");
    a.insert(2, "goodbye");

    let values: Vec<&str> = a.values().cloned().collect();
    assert_eq!(values, ["hello", "goodbye"]);
    ```
    */
    pub fn values(&self) -> Values<'_, A> {
        match self {
            TinyMap::Stack(map) => Values::Stack(map.values()),
            TinyMap::Heap(map) => Values::Heap(map.values()),
        }
    }
    /**
    Gets a mutable iterator over the values of the map, sorted

    # Example

    ```
    use tinymap::TinyMap;

    let mut a = TinyMap::<[(i32, String); 10]>::new();
    a.insert(1, String::from("hello"));
    a.insert(2, String::from("goodbye"));

    for value in a.values_mut() {
        value.push_str("!");
    }

    let values: Vec<String> = a.values().cloned().collect();
    assert_eq!(values, [String::from("hello!"),
                        String::from("goodbye!")]);
    ```
    */
    pub fn values_mut(&mut self) -> ValuesMut<'_, A> {
        match self {
            TinyMap::Stack(map) => ValuesMut::Stack(map.values_mut()),
            TinyMap::Heap(map) => ValuesMut::Heap(map.values_mut()),
        }
    }
}

impl<A, K, V> TinyMap<A>
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
    use tinymap::TinyMap;

    let mut map = TinyMap::<[(i32, &str); 10]>::new();
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
        match self {
            TinyMap::Stack(map) => map.remove(key),
            TinyMap::Heap(map) => map.remove(key),
        }
    }
}

impl<A, K, V, Q> Index<&Q> for TinyMap<A>
where
    A: Array,
    A::Item: MapEntry<Key = K, Value = V>,
    K: Ord + Borrow<Q>,
    Q: Ord,
{
    type Output = V;
    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key)
            .unwrap_or_else(|| panic!("No entry found for key"))
    }
}

impl<A, K, V> fmt::Debug for TinyMap<A>
where
    A: Array,
    A::Item: MapEntry<Key = K, Value = V>,
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<A> From<ArrayMap<A>> for TinyMap<A>
where
    A: Array,
    A::Item: MapEntry,
{
    fn from(map: ArrayMap<A>) -> Self {
        TinyMap::Stack(map)
    }
}

impl<A, K> From<A> for TinyMap<A>
where
    A: Array,
    A::Item: MapEntry<Key = K>,
    K: Ord,
{
    fn from(array: A) -> Self {
        TinyMap::from(ArrayMap::from(array))
    }
}

/// Elements from the iterator beyond the map's capacity will be discarded.
impl<A, K> FromIterator<A::Item> for TinyMap<A>
where
    A: Array + Default,
    A::Item: MapEntry<Key = K>,
    K: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = A::Item>,
    {
        let mut map = TinyMap::default();
        for (key, value) in iter.into_iter().map(MapEntry::into_pair).take(A::CAPACITY) {
            map.insert(key, value);
        }
        map
    }
}

/// An iterator over references to the key-value pairs in an TinyMap
pub enum Iter<'a, A>
where
    A: Array,
    A::Item: MapEntry,
{
    #[doc(hidden)]
    Stack(crate::array_map::Iter<'a, A>),
    #[doc(hidden)]
    Heap(
        std::collections::btree_map::Iter<
            'a,
            <A::Item as MapEntry>::Key,
            <A::Item as MapEntry>::Value,
        >,
    ),
}

impl<'a, A, K, V> Iterator for Iter<'a, A>
where
    A: Array,
    A::Item: MapEntry<Key = K, Value = V>,
    K: 'a,
    V: 'a,
{
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Stack(iter) => iter.next().map(|entry| (entry.key(), entry.value())),
            Iter::Heap(iter) => iter.next(),
        }
    }
}

/// An iterator over references to keys and mutable references to values in an TinyMap
pub enum IterMut<'a, A>
where
    A: Array,
    A::Item: MapEntry,
{
    #[doc(hidden)]
    Stack(crate::array_map::IterMut<'a, A>),
    #[doc(hidden)]
    Heap(
        std::collections::btree_map::IterMut<
            'a,
            <A::Item as MapEntry>::Key,
            <A::Item as MapEntry>::Value,
        >,
    ),
}

impl<'a, A, K, V> Iterator for IterMut<'a, A>
where
    A: Array,
    A::Item: MapEntry<Key = K, Value = V>,
    K: 'a,
    V: 'a,
{
    type Item = (&'a K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterMut::Stack(iter) => iter.next(),
            IterMut::Heap(iter) => iter.next(),
        }
    }
}

/// An iterator over references to the keys in an TinyMap
pub enum Keys<'a, A>
where
    A: Array,
    A::Item: MapEntry,
{
    #[doc(hidden)]
    Stack(crate::array_map::Keys<'a, A>),
    #[doc(hidden)]
    Heap(
        std::collections::btree_map::Keys<
            'a,
            <A::Item as MapEntry>::Key,
            <A::Item as MapEntry>::Value,
        >,
    ),
}

impl<'a, A, K> Iterator for Keys<'a, A>
where
    A: Array,
    A::Item: MapEntry<Key = K>,
    K: 'a,
{
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Keys::Stack(iter) => iter.next(),
            Keys::Heap(iter) => iter.next(),
        }
    }
}

/// An iterator over references to the values in an TinyMap
pub enum Values<'a, A>
where
    A: Array,
    A::Item: MapEntry,
{
    #[doc(hidden)]
    Stack(crate::array_map::Values<'a, A>),
    #[doc(hidden)]
    Heap(
        std::collections::btree_map::Values<
            'a,
            <A::Item as MapEntry>::Key,
            <A::Item as MapEntry>::Value,
        >,
    ),
}

impl<'a, A, V> Iterator for Values<'a, A>
where
    A: Array,
    A::Item: MapEntry<Value = V>,
    V: 'a,
{
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Values::Stack(iter) => iter.next(),
            Values::Heap(iter) => iter.next(),
        }
    }
}

/// An iterator over mutable references to the values in an TinyMap
pub enum ValuesMut<'a, A>
where
    A: Array,
    A::Item: MapEntry,
{
    #[doc(hidden)]
    Stack(crate::array_map::ValuesMut<'a, A>),
    #[doc(hidden)]
    Heap(
        std::collections::btree_map::ValuesMut<
            'a,
            <A::Item as MapEntry>::Key,
            <A::Item as MapEntry>::Value,
        >,
    ),
}

impl<'a, A, V> Iterator for ValuesMut<'a, A>
where
    A: Array,
    A::Item: MapEntry<Value = V>,
    V: 'a,
{
    type Item = &'a mut V;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ValuesMut::Stack(iter) => iter.next(),
            ValuesMut::Heap(iter) => iter.next(),
        }
    }
}
