//! A map that starts on the stack but can automatically move to the heap

use core::{borrow::Borrow, fmt, iter::FromIterator, mem::swap, ops::Index};
use std::collections::{btree_map, BTreeMap};

use crate::{ArrayMap, MapArray};

/**
A map that starts on the stack but can automatically move to the heap
*/
pub enum TinyMap<A>
where
    A: MapArray,
{
    /// An map with items on the stack
    Stack(ArrayMap<A>),
    /// A map with items on the heap
    Heap(BTreeMap<A::Key, A::Value>),
}

impl<A> Default for TinyMap<A>
where
    A: MapArray,
{
    fn default() -> Self {
        TinyMap::Stack(ArrayMap::default())
    }
}

impl<A> Clone for TinyMap<A>
where
    A: MapArray,
    A::Key: Clone,
    A::Value: Clone,
{
    fn clone(&self) -> Self {
        match self {
            TinyMap::Stack(map) => TinyMap::Stack(map.clone()),
            TinyMap::Heap(map) => TinyMap::Heap(map.clone()),
        }
    }
}

impl<A> TinyMap<A>
where
    A: MapArray,
{
    /**
    Creates a new empty TinyMap

    # Example
    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(i32, &str)>; 2]>::new();

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
    use tinymap::*;

    let mut a = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
    a.insert(1, "a");
    a.clear();
    assert!(a.is_empty());
    ```
    */
    pub fn clear(&mut self) {
        *self = Self::new();
    }
    /**
    Returns the number of elements in the map

    # Example

    ```
    use tinymap::*;

    let mut a = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
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
    use tinymap::*;

    let mut a = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
    assert!(a.is_empty());
    a.insert(1, "a");
    assert!(!a.is_empty());
    ```
    */
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /**
    Returns the maximum number of elements the map can contain on the stack

    # Example

    ```
    use tinymap::*;

    let mut a = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
    assert_eq!(2, a.capacity());
    ```
    */
    pub fn capacity(&self) -> usize {
        A::CAPACITY
    }
    /**
    Gets an iterator over the entries of the map, sorted by key

    # Example

    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
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
    pub fn iter(&self) -> Iter<'_, A::Key, A::Value> {
        match self {
            TinyMap::Stack(map) => Iter::Stack(map.iter()),
            TinyMap::Heap(map) => Iter::Heap(map.iter()),
        }
    }
}

impl<A> TinyMap<A>
where
    A: MapArray,
    A::Key: Ord,
{
    /**
    Inserts a key-value pair into the map

    If the map did not have this key present, None is returned.

    If the map did have this key present, the key and value are updated, and the old value is returned.

    # Panics

    Panics if insertion would cause the map to excede its capacity.

    # Example

    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.is_empty(), false);

    map.insert(37, "b");
    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");
    ```
    */
    pub fn insert(&mut self, key: A::Key, value: A::Value) -> Option<A::Value> {
        match self {
            TinyMap::Stack(map) => match map.try_insert(key, value) {
                Ok(res) => res,
                Err((k, v)) => {
                    let mut replacement_map = ArrayMap::default();
                    swap(&mut replacement_map, map);
                    let mut btree_map = BTreeMap::new();
                    for (k, v) in replacement_map {
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
    /**
    Returns a reference to the value corresponding to the key

    # Example

    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), None);
    ```
    */
    pub fn get<Q>(&self, key: &Q) -> Option<&A::Value>
    where
        A::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match self {
            TinyMap::Stack(map) => map.get(key),
            TinyMap::Heap(map) => map.get(key),
        }
    }
    /**
    Returns true if the map contains a value for the specified key

    # Example

    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
    map.insert(1, "a");
    assert_eq!(map.contains_key(&1), true);
    assert_eq!(map.contains_key(&2), false);
    ```
    */
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        A::Key: Borrow<Q>,
        Q: Ord + ?Sized,
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
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
    map.insert(1, "a");
    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }
    assert_eq!(map[&1], "b");
    ```
    */
    pub fn get_mut<'a, Q>(&'a mut self, key: &Q) -> Option<&'a mut A::Value>
    where
        A::Key: Borrow<Q> + 'a,
        Q: Ord + ?Sized,
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
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(&str, i32)>; 2]>::new();
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
    pub fn iter_mut(&mut self) -> IterMut<'_, A::Key, A::Value> {
        match self {
            TinyMap::Stack(map) => IterMut::Stack(map.iter_mut()),
            TinyMap::Heap(map) => IterMut::Heap(map.iter_mut()),
        }
    }
    /**
    Gets an iterator over the keys of the map, sorted

    # Example

    ```
    use tinymap::*;

    let mut a = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
    a.insert(2, "b");
    a.insert(1, "a");

    let keys: Vec<_> = a.keys().cloned().collect();
    assert_eq!(keys, [1, 2]);
    ```
    */
    pub fn keys(&self) -> Keys<'_, A::Key, A::Value> {
        match self {
            TinyMap::Stack(map) => Keys::Stack(map.keys()),
            TinyMap::Heap(map) => Keys::Heap(map.keys()),
        }
    }
    /**
    Gets an iterator over the values of the map, sorted

    # Example

    ```
    use tinymap::*;

    let mut a = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
    a.insert(1, "hello");
    a.insert(2, "goodbye");

    let values: Vec<&str> = a.values().cloned().collect();
    assert_eq!(values, ["hello", "goodbye"]);
    ```
    */
    pub fn values(&self) -> Values<'_, A::Key, A::Value> {
        match self {
            TinyMap::Stack(map) => Values::Stack(map.values()),
            TinyMap::Heap(map) => Values::Heap(map.values()),
        }
    }
    /**
    Gets a mutable iterator over the values of the map, sorted

    # Example

    ```
    use tinymap::*;

    let mut a = TinyMap::<[Inner<(i32, String)>; 2]>::new();
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
    pub fn values_mut(&mut self) -> ValuesMut<'_, A::Key, A::Value> {
        match self {
            TinyMap::Stack(map) => ValuesMut::Stack(map.values_mut()),
            TinyMap::Heap(map) => ValuesMut::Heap(map.values_mut()),
        }
    }
    /**
    Removes a key from the map, returning the value at the key if the key was previously in the map

    # Example

    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(i32, &str)>; 2]>::new();
    map.insert(1, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);
    ```
    */
    pub fn remove<Q>(&mut self, key: &Q) -> Option<A::Value>
    where
        A::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match self {
            TinyMap::Stack(map) => map.remove(key),
            TinyMap::Heap(map) => map.remove(key),
        }
    }
    /**
    Gets the given key's corresponding entry in the map for in-place manipulation

    # Example

    ```
    use tinymap::*;

    let mut count = TinyMap::<[Inner<(&str, i32)>; 10]>::new();

    // count the number of occurrences of letters in the vec
    for x in vec!["a","b","a","c","a","b"] {
        *count.entry(x).or_insert(0) += 1;
    }

    assert_eq!(count["a"], 3);
    ```
    */
    pub fn entry(&mut self, key: A::Key) -> Entry<'_, A> {
        match self {
            TinyMap::Stack(map) => Entry::Stack(map.entry(key)),
            TinyMap::Heap(map) => Entry::Heap(map.entry(key)),
        }
    }
}

impl<A, Q> Index<&Q> for TinyMap<A>
where
    A: MapArray,
    A::Key: Ord + Borrow<Q>,
    Q: Ord + ?Sized,
{
    type Output = A::Value;
    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key)
            .unwrap_or_else(|| panic!("No entry found for key"))
    }
}

impl<A> fmt::Debug for TinyMap<A>
where
    A: MapArray,
    A::Key: fmt::Debug,
    A::Value: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<A> From<ArrayMap<A>> for TinyMap<A>
where
    A: MapArray,
{
    fn from(map: ArrayMap<A>) -> Self {
        TinyMap::Stack(map)
    }
}

impl<A> From<BTreeMap<A::Key, A::Value>> for TinyMap<A>
where
    A: MapArray,
{
    fn from(map: BTreeMap<A::Key, A::Value>) -> Self {
        TinyMap::Heap(map)
    }
}

impl<A> From<A> for TinyMap<A>
where
    A: MapArray,
    A::Key: Ord,
{
    fn from(array: A) -> Self {
        TinyMap::from(ArrayMap::from(array))
    }
}

impl<A> PartialEq for TinyMap<A>
where
    A: MapArray,
    A::Key: PartialEq,
    A::Value: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<A> Eq for TinyMap<A>
where
    A: MapArray,
    A::Key: Eq,
    A::Value: Eq,
{
}

impl<'a, A> IntoIterator for &'a TinyMap<A>
where
    A: MapArray,
{
    type Item = (&'a A::Key, &'a A::Value);
    type IntoIter = Iter<'a, A::Key, A::Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A> IntoIterator for &'a mut TinyMap<A>
where
    A: MapArray,
    A::Key: Ord,
{
    type Item = (&'a A::Key, &'a mut A::Value);
    type IntoIter = IterMut<'a, A::Key, A::Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(feature = "alloc")]
impl<A> IntoIterator for TinyMap<A>
where
    A: MapArray,
{
    type Item = (A::Key, A::Value);
    type IntoIter = IntoIter<A::Key, A::Value>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            TinyMap::Stack(map) => IntoIter::Stack(map.into_iter()),
            TinyMap::Heap(map) => IntoIter::Heap(map.into_iter()),
        }
    }
}

/// Elements from the iterator beyond the map's capacity will be discarded.
impl<A> FromIterator<(A::Key, A::Value)> for TinyMap<A>
where
    A: MapArray,
    A::Key: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (A::Key, A::Value)>,
    {
        let mut map = TinyMap::default();
        for (key, value) in iter.into_iter().take(A::CAPACITY) {
            map.insert(key, value);
        }
        map
    }
}

impl<A> Extend<(A::Key, A::Value)> for TinyMap<A>
where
    A: MapArray,
    A::Key: Ord,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (A::Key, A::Value)>,
    {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

/// An consuming iterator over the values in an ArraySet
pub enum IntoIter<K, V> {
    #[doc(hidden)]
    Stack(crate::array_map::IntoIter<K, V>),
    #[doc(hidden)]
    Heap(btree_map::IntoIter<K, V>),
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IntoIter::Stack(iter) => iter.next(),
            IntoIter::Heap(iter) => iter.next(),
        }
    }
}

/// An iterator over references to the key-value pairs in an TinyMap
pub enum Iter<'a, K, V> {
    #[doc(hidden)]
    Stack(crate::array_map::Iter<'a, K, V>),
    #[doc(hidden)]
    Heap(btree_map::Iter<'a, K, V>),
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Stack(iter) => iter.next(),
            Iter::Heap(iter) => iter.next(),
        }
    }
}

/// An iterator over references to keys and mutable references to values in an TinyMap
pub enum IterMut<'a, K, V> {
    #[doc(hidden)]
    Stack(crate::array_map::IterMut<'a, K, V>),
    #[doc(hidden)]
    Heap(btree_map::IterMut<'a, K, V>),
}

impl<'a, K, V> Iterator for IterMut<'a, K, V>
where
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
pub enum Keys<'a, K, V> {
    #[doc(hidden)]
    Stack(crate::array_map::Keys<'a, K, V>),
    #[doc(hidden)]
    Heap(btree_map::Keys<'a, K, V>),
}

impl<'a, K, V> Iterator for Keys<'a, K, V>
where
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
pub enum Values<'a, K, V> {
    #[doc(hidden)]
    Stack(crate::array_map::Values<'a, K, V>),
    #[doc(hidden)]
    Heap(btree_map::Values<'a, K, V>),
}

impl<'a, K, V> Iterator for Values<'a, K, V>
where
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
pub enum ValuesMut<'a, K, V> {
    #[doc(hidden)]
    Stack(crate::array_map::ValuesMut<'a, K, V>),
    #[doc(hidden)]
    Heap(btree_map::ValuesMut<'a, K, V>),
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V>
where
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

/// A view into a single entry in a map, which may either be vacant or occupied.
pub enum Entry<'a, A>
where
    A: MapArray,
{
    /// An entry into a map on the stack
    Stack(crate::array_map::Entry<'a, A>),
    /// An entry into a map on the heap
    Heap(btree_map::Entry<'a, A::Key, A::Value>),
}

impl<'a, A> Entry<'a, A>
where
    A: MapArray,
    A::Key: Ord,
{
    /**
    Ensures a value is in the entry by inserting the default if empty,
    and returns a mutable reference to the value in the entry.

    # Panics

    Panics if insertion would cause the map to excede its capacity.

    # Example

    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(&str, i32)>; 10]>::new();
    map.entry("poneyland").or_insert(12);

    assert_eq!(map["poneyland"], 12);
    ```
    */
    pub fn or_insert(self, default: A::Value) -> &'a mut A::Value {
        match self {
            Entry::Stack(entry) => entry.or_insert(default),
            Entry::Heap(entry) => entry.or_insert(default),
        }
    }
    /**
    Ensures a value is in the entry by inserting the result of the default function if empty,
    and returns a mutable reference to the value in the entry.

    # Panics

    Panics if insertion would cause the map to excede its capacity.

    # Example

    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(&str, String)>; 10]>::new();
    let s = "hoho";

    map.entry("poneyland").or_insert_with(|| s.to_string());

    assert_eq!(map["poneyland"], "hoho".to_string());
    ```
    */
    pub fn or_insert_with<F>(self, default: F) -> &'a mut A::Value
    where
        F: FnOnce() -> A::Value,
    {
        match self {
            Entry::Stack(entry) => entry.or_insert_with(default),
            Entry::Heap(entry) => entry.or_insert_with(default),
        }
    }
    /**
    Returns a reference to this entry's key.

    # Example

    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(&str, usize)>; 10]>::new();
    assert_eq!(map.entry("poneyland").key(), &"poneyland");
    ```
    */
    pub fn key(&self) -> &A::Key {
        match self {
            Entry::Stack(entry) => entry.key(),
            Entry::Heap(entry) => entry.key(),
        }
    }
    /**
    Provides in-place mutable access to an occupied entry before any potential inserts into the map.

    # Example

    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(&str, usize)>; 10]>::new();

    map.entry("poneyland")
        .and_modify(|e| { *e += 1 })
        .or_insert(42);
    assert_eq!(map["poneyland"], 42);

    map.entry("poneyland")
        .and_modify(|e| { *e += 1 })
        .or_insert(42);
    assert_eq!(map["poneyland"], 43);
    ```
    */
    pub fn and_modify<F>(self, f: F) -> Entry<'a, A>
    where
        F: FnOnce(&mut A::Value),
    {
        match self {
            Entry::Stack(entry) => Entry::Stack(entry.and_modify(f)),
            Entry::Heap(entry) => Entry::Heap(entry.and_modify(f)),
        }
    }
}

impl<'a, A> Entry<'a, A>
where
    A: MapArray,
    A::Key: Ord,
    A::Value: Default,
{
    /**
    Ensures a value is in the entry by inserting the default value if empty,
    and returns a mutable reference to the value in the entry.

    # Example

    ```
    use tinymap::*;

    let mut map = TinyMap::<[Inner<(&str, Option<usize>)>; 10]>::new();

    map.entry("poneyland").or_default();

    assert_eq!(map["poneyland"], None);
    ```
    */
    pub fn or_default(self) -> &'a mut A::Value {
        self.or_insert_with(Default::default)
    }
}
