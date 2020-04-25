//! An array-backed, map-like data structure

use core::{
    borrow::Borrow,
    fmt,
    iter::FromIterator,
    mem::{replace, swap, zeroed},
    ops::Index,
};

use crate::{Inner, MapArray};

/**
An array-backed, map-like data structure

ArrayMap wraps an array of key-value pairs and supports operation similar to a BTreeMap or HashMap.
It has a fixed capacity, but it keeps track of how many pairs have been inserted and removed.
*/
pub struct ArrayMap<A>
where
    A: MapArray,
{
    array: A,
    len: usize,
}

impl<A> Default for ArrayMap<A>
where
    A: MapArray,
{
    fn default() -> Self {
        ArrayMap {
            array: unsafe { zeroed() },
            len: 0,
        }
    }
}

impl<A> Clone for ArrayMap<A>
where
    A: MapArray,
    A::Key: Clone,
    A::Value: Clone,
{
    fn clone(&self) -> Self {
        let mut array: A = unsafe { zeroed() };
        let len = self.len;
        for (i, (k, v)) in self.iter().enumerate() {
            array.as_mut_slice()[i] = Inner::new((k.clone(), v.clone()));
        }
        ArrayMap { array, len }
    }
}

impl<A> ArrayMap<A>
where
    A: MapArray,
{
    /**
    Returns the number of elements in the map

    # Example

    ```
    use tinymap::*;

    let mut a = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
    assert_eq!(a.len(), 0);
    a.insert(1, "a");
    assert_eq!(a.len(), 1);
    ```
    */
    pub fn len(&self) -> usize {
        self.len
    }
    /**
    Returns `true` if the map contains no elements

    # Example

    ```
    use tinymap::*;

    let mut a = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
    assert!(a.is_empty());
    a.insert(1, "a");
    assert!(!a.is_empty());
    ```
    */
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /**
    Creates a new empty ArrayMap

    # Example
    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();

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

    let mut a = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
    a.insert(1, "a");
    a.clear();
    assert!(a.is_empty());
    ```
    */
    pub fn clear(&mut self) {
        *self = Self::new();
    }
    /**
    Returns the maximum number of elements the map can contain

    # Example

    ```
    use tinymap::*;

    let mut a = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
    assert_eq!(10, a.capacity());
    ```
    */
    pub fn capacity(&self) -> usize {
        A::CAPACITY
    }
    fn find<Q>(&self, key: &Q) -> Result<usize, usize>
    where
        A::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.array.as_slice()[..self.len].binary_search_by_key(&key.borrow(), |entry| {
            unsafe { entry.as_ptr().as_ref() }.unwrap().0.borrow()
        })
    }
    /**
    Returns true if the map contains a value for the specified key

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
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
        self.find(key).is_ok()
    }
    /**
    Returns a reference to the value corresponding to the key

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
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
        if let Ok(i) = self.find(key) {
            Some(
                &unsafe { self.array.as_slice()[i].as_ptr().as_ref() }
                    .unwrap()
                    .1,
            )
        } else {
            None
        }
    }
    /**
    Returns a mutable reference to the value corresponding to the key

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
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
        if let Ok(i) = self.find(key) {
            Some(
                &mut unsafe { self.array.as_mut_slice()[i].as_mut_ptr().as_mut() }
                    .unwrap()
                    .1,
            )
        } else {
            None
        }
    }
    /**
    Gets an iterator over the entries of the map, sorted by key

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
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
        Iter {
            iter: self.array.as_slice()[..self.len].iter(),
        }
    }
    /**
    Gets a mutable iterator over the entries of the map, sorted by key

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(&str, i32)>; 10]>::new();
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
        IterMut {
            iter: self.array.as_mut_slice()[..self.len].iter_mut(),
        }
    }
    /**
    Gets an iterator over the keys of the map, sorted

    # Example

    ```
    use tinymap::*;

    let mut a = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
    a.insert(2, "b");
    a.insert(1, "a");

    let keys: Vec<_> = a.keys().cloned().collect();
    assert_eq!(keys, [1, 2]);
    ```
    */
    pub fn keys(&self) -> Keys<'_, A::Key, A::Value> {
        Keys {
            iter: self.array.as_slice()[..self.len].iter(),
        }
    }
    /**
    Gets an iterator over the values of the map, sorted

    # Example

    ```
    use tinymap::*;

    let mut a = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
    a.insert(1, "hello");
    a.insert(2, "goodbye");

    let values: Vec<&str> = a.values().cloned().collect();
    assert_eq!(values, ["hello", "goodbye"]);
    ```
    */
    pub fn values(&self) -> Values<'_, A::Key, A::Value> {
        Values {
            iter: self.array.as_slice()[..self.len].iter(),
        }
    }
    /**
    Gets a mutable iterator over the values of the map, sorted

    # Example

    ```
    use tinymap::*;

    let mut a = ArrayMap::<[Inner<(i32, String)>; 10]>::new();
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
        ValuesMut {
            iter: self.array.as_mut_slice()[..self.len].iter_mut(),
        }
    }
}

impl<A> ArrayMap<A>
where
    A: MapArray + Copy,
{
    /**
    Get a copy of this map

    Because of the way its internals work, ArrayMap must implement `Drop`, so it cannot implement `Copy`.

    However, that does not make a copy any less trivial with the right array type, hence this function.

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(&str, i32)>; 10]>::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    let copy = map.copy();
    assert_eq!(3, copy.len());
    ```
    */
    pub fn copy(&self) -> Self {
        ArrayMap {
            array: self.array,
            len: self.len,
        }
    }
}

impl<A> ArrayMap<A>
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

    let mut map = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.is_empty(), false);

    map.insert(37, "b");
    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");
    ```
    */
    pub fn insert(&mut self, key: A::Key, value: A::Value) -> Option<A::Value> {
        self.try_insert(key, value)
            .unwrap_or_else(|_| panic!("Insertion would excede capacity"))
    }
    /**
    Attempts to insert a key-value pair into the map

    If the map did not have this key present, None is returned.

    If the map did have this key present, the key and value are updated, and the old value is returned.

    # Errors

    If insertion would cause the map to excede its capacity, this function returns an error containing
    the key-value pair that could not be inserted.

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(i32, &str)>; 3]>::new();
    assert!(map.try_insert(37, "a").is_ok());
    assert!(map.try_insert(2, "b").is_ok());
    assert!(map.try_insert(16, "c").is_ok());
    assert!(map.try_insert(0, "d").is_err());
    ```
    */
    pub fn try_insert(
        &mut self,
        key: A::Key,
        value: A::Value,
    ) -> Result<Option<A::Value>, (A::Key, A::Value)> {
        self.try_insert_index(key, value, None)
    }
    fn try_insert_index(
        &mut self,
        key: A::Key,
        value: A::Value,
        index: Option<usize>,
    ) -> Result<Option<A::Value>, (A::Key, A::Value)> {
        if self.len == A::CAPACITY {
            return Err((key, value));
        }
        let i = if let Some(index) = index {
            Err(index)
        } else {
            self.find(&key)
        };
        match i {
            Ok(i) => {
                let mut entry = Inner::new((key, value));
                swap(&mut entry, &mut self.array.as_mut_slice()[i]);
                Ok(Some(unsafe { entry.assume_init() }.1))
            }
            Err(i) => {
                let slice = self.array.as_mut_slice();
                for j in ((i + 1)..=self.len).rev() {
                    slice.swap(j - 1, j);
                }
                let mut entry = Inner::new((key, value));
                swap(&mut entry, &mut slice[i]);
                self.len += 1;
                Ok(None)
            }
        }
    }
    fn remove_index(&mut self, i: usize) -> (A::Key, A::Value) {
        let slice = self.array.as_mut_slice();
        let mut entry = Inner::uninit();
        swap(&mut entry, &mut slice[i]);
        for j in (i + 1)..self.len {
            slice.swap(j - 1, j);
        }
        self.len -= 1;
        unsafe { entry.assume_init() }
    }
    /**
    Removes a key from the map, returning the value at the key if the key was previously in the map

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(i32, &str)>; 10]>::new();
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
        if let Ok(i) = self.find(key) {
            Some(self.remove_index(i).1)
        } else {
            None
        }
    }
    /**
    Gets the given key's corresponding entry in the map for in-place manipulation

    # Example

    ```
    use tinymap::*;

    let mut count = ArrayMap::<[Inner<(&str, i32)>; 10]>::new();

    // count the number of occurrences of letters in the vec
    for x in vec!["a","b","a","c","a","b"] {
        *count.entry(x).or_insert(0) += 1;
    }

    assert_eq!(count["a"], 3);
    ```
    */
    pub fn entry(&mut self, key: A::Key) -> Entry<'_, A> {
        match self.find(&key) {
            Ok(i) => Entry::Occupied(OccupiedEntry {
                map: self,
                index: i,
            }),
            Err(i) => Entry::Vacant(VacantEntry {
                map: self,
                index: i,
                key,
            }),
        }
    }
}

impl<A, Q> Index<&Q> for ArrayMap<A>
where
    A: MapArray,
    A::Key: Borrow<Q>,
    Q: Ord + ?Sized,
{
    type Output = A::Value;
    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key)
            .unwrap_or_else(|| panic!("No entry found for key"))
    }
}

impl<A> fmt::Debug for ArrayMap<A>
where
    A: MapArray,
    A::Key: fmt::Debug,
    A::Value: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.iter().map(|(k, v)| (k, v)))
            .finish()
    }
}

impl<A> From<A> for ArrayMap<A>
where
    A: MapArray,
    A::Key: Ord,
{
    fn from(mut array: A) -> Self {
        array.as_mut_slice().sort_unstable_by(|a, b| {
            unsafe { a.as_ptr().as_ref() }
                .unwrap()
                .0
                .cmp(&unsafe { b.as_ptr().as_ref() }.unwrap().0)
        });
        ArrayMap {
            array,
            len: A::CAPACITY,
        }
    }
}

impl<A> PartialEq for ArrayMap<A>
where
    A: MapArray,
    A::Key: PartialEq,
    A::Value: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<A> Eq for ArrayMap<A>
where
    A: MapArray,
    A::Key: Eq,
    A::Value: Eq,
{
}

impl<'a, A> IntoIterator for &'a ArrayMap<A>
where
    A: MapArray,
{
    type Item = (&'a A::Key, &'a A::Value);
    type IntoIter = Iter<'a, A::Key, A::Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A> IntoIterator for &'a mut ArrayMap<A>
where
    A: MapArray,
{
    type Item = (&'a A::Key, &'a mut A::Value);
    type IntoIter = IterMut<'a, A::Key, A::Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(feature = "alloc")]
impl<A> IntoIterator for ArrayMap<A>
where
    A: MapArray,
{
    type Item = (A::Key, A::Value);
    type IntoIter = IntoIter<A::Key, A::Value>;
    fn into_iter(mut self) -> Self::IntoIter {
        let array = core::mem::replace(&mut self.array, unsafe { zeroed() });
        IntoIter {
            iter: array.into_boxed_slice().into_vec().into_iter(),
        }
    }
}

/// Elements from the iterator beyond the map's capacity will be discarded.
impl<A> FromIterator<(A::Key, A::Value)> for ArrayMap<A>
where
    A: MapArray,
    A::Key: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (A::Key, A::Value)>,
    {
        let mut map = ArrayMap::default();
        for (key, value) in iter.into_iter().take(A::CAPACITY) {
            map.insert(key, value);
        }
        map
    }
}

/// Elements from the iterator beyond the map's capacity will be discarded.
impl<A> Extend<(A::Key, A::Value)> for ArrayMap<A>
where
    A: MapArray,
    A::Key: Ord,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (A::Key, A::Value)>,
    {
        for (k, v) in iter {
            if self.try_insert(k, v).is_err() {
                break;
            }
        }
    }
}

impl<A> Drop for ArrayMap<A>
where
    A: MapArray,
{
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                self.array.as_mut_slice()[i].as_mut_ptr().drop_in_place();
            }
        }
    }
}

/// An consuming iterator over the key-value pairs in an ArrayMap
#[cfg(feature = "alloc")]
pub struct IntoIter<K, V> {
    iter: std::vec::IntoIter<Inner<(K, V)>>,
}

#[cfg(feature = "alloc")]
impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|entry| unsafe { entry.assume_init() })
    }
}

/// An iterator over references to the key-value pairs in an ArrayMap
pub struct Iter<'a, K, V> {
    iter: core::slice::Iter<'a, Inner<(K, V)>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|entry| {
            let pair = unsafe { entry.as_ptr().as_ref() }.unwrap();
            (&pair.0, &pair.1)
        })
    }
}

/// An iterator over references to keys and mutable references to values in an ArrayMap
pub struct IterMut<'a, K, V> {
    iter: core::slice::IterMut<'a, Inner<(K, V)>>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    type Item = (&'a K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|entry| {
            let pair = unsafe { entry.as_mut_ptr().as_mut() }.unwrap();
            (&pair.0, &mut pair.1)
        })
    }
}

/// An iterator over references to the keys in an ArrayMap
pub struct Keys<'a, K, V> {
    iter: core::slice::Iter<'a, Inner<(K, V)>>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V>
where
    K: 'a,
{
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|entry| &unsafe { entry.as_ptr().as_ref() }.unwrap().0)
    }
}

/// An iterator over references to the values in an ArrayMap
pub struct Values<'a, K, V> {
    iter: core::slice::Iter<'a, Inner<(K, V)>>,
}

impl<'a, K, V> Iterator for Values<'a, K, V>
where
    V: 'a,
{
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|entry| &unsafe { entry.as_ptr().as_ref() }.unwrap().1)
    }
}

/// An iterator over mutable references to the values in an ArrayMap
pub struct ValuesMut<'a, K, V> {
    iter: core::slice::IterMut<'a, Inner<(K, V)>>,
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V>
where
    V: 'a,
{
    type Item = &'a mut V;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|entry| &mut unsafe { entry.as_mut_ptr().as_mut() }.unwrap().1)
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied.
pub enum Entry<'a, A>
where
    A: MapArray,
{
    /// A vacant entry.
    Vacant(VacantEntry<'a, A>),
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, A>),
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

    let mut map = ArrayMap::<[Inner<(&str, i32)>; 10]>::new();
    map.entry("poneyland").or_insert(12);

    assert_eq!(map["poneyland"], 12);
    ```
    */
    pub fn or_insert(self, default: A::Value) -> &'a mut A::Value {
        match self {
            Entry::Vacant(entry) => entry.insert(default),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }
    /**
    Attempts to ensure a value is in the entry by inserting the default if empty,
    and returns a mutable reference to the value in the entry.

    # Errors

    If insertion would cause the map to excede its capacity, this function returns an error containing
    the key-value pair that could not be inserted.

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(&str, i32)>; 2]>::new();
    assert!(map.entry("poneyland").or_try_insert(12).is_ok());

    assert_eq!(map["poneyland"], 12);

    assert!(map.entry("donkeyville").or_try_insert(20).is_ok());
    assert!(map.entry("muledale").or_try_insert(7).is_err());
    ```
    */
    pub fn or_try_insert(self, default: A::Value) -> Result<&'a mut A::Value, (A::Key, A::Value)> {
        match self {
            Entry::Vacant(entry) => entry.try_insert(default),
            Entry::Occupied(entry) => Ok(entry.into_mut()),
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

    let mut map = ArrayMap::<[Inner<(&str, String)>; 10]>::new();
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
            Entry::Vacant(entry) => entry.insert(default()),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }
    /**
    Attempts to ensure a value is in the entry by inserting the result of the default function if empty,
    and returns a mutable reference to the value in the entry.

    # Errors

    If insertion would cause the map to excede its capacity, this function returns an error containing
    the key-value pair that could not be inserted.

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(&str, String)>; 2]>::new();
    let s = "hoho";

    assert!(map.entry("poneyland").or_try_insert_with(|| s.to_string()).is_ok());

    assert_eq!(map["poneyland"], "hoho".to_string());

    assert!(map.entry("donkeyville").or_try_insert_with(|| "wewe".to_string()).is_ok());

    assert!(map.entry("muledale").or_try_insert_with(|| "bubu".to_string()).is_err());
    ```
    */
    pub fn or_try_insert_with<F>(self, default: F) -> Result<&'a mut A::Value, (A::Key, A::Value)>
    where
        F: FnOnce() -> A::Value,
    {
        match self {
            Entry::Vacant(entry) => entry.try_insert(default()),
            Entry::Occupied(entry) => Ok(entry.into_mut()),
        }
    }
    /**
    Returns a reference to this entry's key.

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(&str, usize)>; 10]>::new();
    assert_eq!(map.entry("poneyland").key(), &"poneyland");
    ```
    */
    pub fn key(&self) -> &A::Key {
        match self {
            Entry::Vacant(entry) => entry.key(),
            Entry::Occupied(entry) => entry.key(),
        }
    }
    /**
    Provides in-place mutable access to an occupied entry before any potential inserts into the map.

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Inner<(&str, usize)>; 10]>::new();

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
    pub fn and_modify<F>(mut self, f: F) -> Entry<'a, A>
    where
        F: FnOnce(&mut A::Value),
    {
        if let Entry::Occupied(entry) = &mut self {
            f(entry.get_mut());
        }
        self
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

    let mut map = ArrayMap::<[Inner<(&str, Option<usize>)>; 10]>::new();

    map.entry("poneyland").or_default();

    assert_eq!(map["poneyland"], None);
    ```
    */
    pub fn or_default(self) -> &'a mut A::Value {
        self.or_insert_with(Default::default)
    }
}

/// An entry in an ArrayMap that is vacant
pub struct VacantEntry<'a, A>
where
    A: MapArray,
{
    map: &'a mut ArrayMap<A>,
    index: usize,
    key: A::Key,
}

impl<'a, A> VacantEntry<'a, A>
where
    A: MapArray + 'a,
    A::Key: Ord,
{
    /**
    Gets a reference to the key that would be used when inserting a value through the VacantEntry.
    */
    pub fn key(&self) -> &A::Key {
        &self.key
    }
    /**
    Take ownership of the key.
    */
    pub fn into_key(self) -> A::Key {
        self.key
    }
    /**
    Sets the value of the entry with the VacantEntry's key and returns a mutable reference to it.

    # Panics

    Panics if insertion would cause the map to excede its capacity.
    */
    pub fn insert(self, value: A::Value) -> &'a mut A::Value {
        self.map
            .try_insert_index(self.key, value, Some(self.index))
            .unwrap_or_else(|_| panic!("Insertion would excede capacity"));
        unsafe {
            self.map.array.as_mut_slice()[self.index]
                .as_mut_ptr()
                .as_mut()
        }
        .map(|(_, v)| v)
        .unwrap()
    }
    /**
    Attempts to set the value of the entry with the VacantEntry's key and return a mutable reference to it.

    # Errors

    If insertion would cause the map to excede its capacity, this function returns an error containing
    the key-value pair that could not be inserted.
    */
    pub fn try_insert(self, value: A::Value) -> Result<&'a mut A::Value, (A::Key, A::Value)> {
        let index = self.index;
        let map = self.map;
        map.try_insert_index(self.key, value, Some(index)).map(|_| {
            unsafe { map.array.as_mut_slice()[index].as_mut_ptr().as_mut() }
                .map(|(_, v)| v)
                .unwrap()
        })
    }
}

/// An entry in an ArrayMap that is occupied
pub struct OccupiedEntry<'a, A>
where
    A: MapArray,
{
    map: &'a mut ArrayMap<A>,
    index: usize,
}

impl<'a, A> OccupiedEntry<'a, A>
where
    A: MapArray + 'a,
    A::Key: Ord,
{
    fn inner(&self) -> &Inner<(A::Key, A::Value)> {
        &self.map.array.as_slice()[self.index]
    }
    fn inner_mut(&mut self) -> &mut Inner<(A::Key, A::Value)> {
        &mut self.map.array.as_mut_slice()[self.index]
    }
    /**
    Gets a reference to the key in the entry
    */
    pub fn key(&self) -> &A::Key {
        &unsafe { self.inner().as_ptr().as_ref() }.unwrap().0
    }
    /**
    Gets a reference to the value in the entry
    */
    pub fn get(&self) -> &A::Value {
        &unsafe { self.inner().as_ptr().as_ref() }.unwrap().1
    }
    /**
    Gets a mutable reference to the value in the entry
    */
    pub fn get_mut(&mut self) -> &mut A::Value {
        &mut unsafe { self.inner_mut().as_mut_ptr().as_mut() }.unwrap().1
    }
    /**
    Sets the value of the entry and returns the entry's old value.
    */
    pub fn insert(&mut self, value: A::Value) -> A::Value {
        replace(self.get_mut(), value)
    }
    /**
    Converts the entry into a mutable reference to its value.
    */
    pub fn into_mut(mut self) -> &'a mut A::Value {
        unsafe { self.inner_mut().as_mut_ptr().as_mut() }
            .map(|(_, v)| v)
            .unwrap()
    }
    /**
    Takes the value of the entry out of the map, and returns it.
    */
    pub fn remove(self) -> A::Value {
        self.remove_entry().1
    }
    /**
    Takes the key-value pair of the entry out of the map, and returns it.
    */
    pub fn remove_entry(self) -> (A::Key, A::Value) {
        self.map.remove_index(self.index)
    }
}
