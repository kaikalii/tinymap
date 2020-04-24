//! An array-backed, map-like data structure

use core::{
    borrow::Borrow,
    fmt,
    iter::FromIterator,
    mem::{swap, zeroed},
    ops::Index,
};

use crate::{Entry, MapArray};

/**
An array-backed, map-like data structure

ArrayMap wraps an array of key-value pairs and supports operation similar to a BTreeMap or HashMap.
It has a fixed capacity, but it keeps track of how many pairs have been inserted and removed.
*/
#[derive(Clone)]
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

impl<A> ArrayMap<A>
where
    A: MapArray,
{
    /**
    Returns the number of elements in the map

    # Example

    ```
    use tinymap::*;

    let mut a = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
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

    let mut a = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
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

    let mut map = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();

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

    let mut a = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
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

    let mut a = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
    assert_eq!(10, a.capacity());
    ```
    */
    pub fn capacity(&self) -> usize {
        A::CAPACITY
    }
    fn find<Q>(&self, key: &Q) -> Result<usize, usize>
    where
        A::Key: Borrow<Q>,
        Q: Ord,
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

    let mut map = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
    map.insert(1, "a");
    assert_eq!(map.contains_key(&1), true);
    assert_eq!(map.contains_key(&2), false);
    ```
    */
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        A::Key: Borrow<Q>,
        Q: Ord,
    {
        self.find(key).is_ok()
    }
    /**
    Returns a reference to the value corresponding to the key

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), None);
    ```
    */
    pub fn get<Q>(&self, key: &Q) -> Option<&A::Value>
    where
        A::Key: Borrow<Q>,
        Q: Ord,
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

    let mut map = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
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
        Q: Ord,
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

    let mut map = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
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
        Iter {
            iter: self.array.as_slice()[..self.len].iter(),
        }
    }
    /**
    Gets a mutable iterator over the entries of the map, sorted by key

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Entry<(&str, i32)>; 10]>::new();
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
        IterMut {
            iter: self.array.as_mut_slice()[..self.len].iter_mut(),
        }
    }
    /**
    Gets an iterator over the keys of the map, sorted

    # Example

    ```
    use tinymap::*;

    let mut a = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
    a.insert(2, "b");
    a.insert(1, "a");

    let keys: Vec<_> = a.keys().cloned().collect();
    assert_eq!(keys, [1, 2]);
    ```
    */
    pub fn keys(&self) -> Keys<'_, A> {
        Keys {
            iter: self.array.as_slice()[..self.len].iter(),
        }
    }
    /**
    Gets an iterator over the values of the map, sorted

    # Example

    ```
    use tinymap::*;

    let mut a = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
    a.insert(1, "hello");
    a.insert(2, "goodbye");

    let values: Vec<&str> = a.values().cloned().collect();
    assert_eq!(values, ["hello", "goodbye"]);
    ```
    */
    pub fn values(&self) -> Values<'_, A> {
        Values {
            iter: self.array.as_slice()[..self.len].iter(),
        }
    }
    /**
    Gets a mutable iterator over the values of the map, sorted

    # Example

    ```
    use tinymap::*;

    let mut a = ArrayMap::<[Entry<(i32, String)>; 10]>::new();
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

    let mut map = ArrayMap::<[Entry<(&str, i32)>; 10]>::new();
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

    let mut map = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
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

    let mut map = ArrayMap::<[Entry<(i32, &str)>; 3]>::new();
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
        if self.len == A::CAPACITY {
            return Err((key, value));
        }
        match self.find(&key) {
            Ok(i) => {
                let mut entry = Entry::new((key, value));
                swap(&mut entry, &mut self.array.as_mut_slice()[i]);
                Ok(Some(unsafe { entry.assume_init() }.1))
            }
            Err(i) => {
                let slice = self.array.as_mut_slice();
                for j in ((i + 1)..=self.len).rev() {
                    slice.swap(j - 1, j);
                }
                let mut entry = Entry::new((key, value));
                swap(&mut entry, &mut slice[i]);
                self.len += 1;
                Ok(None)
            }
        }
    }
    /**
    Removes a key from the map, returning the value at the key if the key was previously in the map

    # Example

    ```
    use tinymap::*;

    let mut map = ArrayMap::<[Entry<(i32, &str)>; 10]>::new();
    map.insert(1, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);
    ```
    */
    pub fn remove<Q>(&mut self, key: &Q) -> Option<A::Value>
    where
        A::Key: Borrow<Q>,
        Q: Ord,
    {
        if let Ok(i) = self.find(key) {
            let slice = self.array.as_mut_slice();
            let mut entry = Entry::uninit();
            swap(&mut entry, &mut slice[i]);
            for j in (i + 1)..self.len {
                slice.swap(j - 1, j);
            }
            self.len -= 1;
            Some(unsafe { entry.assume_init() }.1)
        } else {
            None
        }
    }
}

impl<A, Q> Index<&Q> for ArrayMap<A>
where
    A: MapArray,
    A::Key: Borrow<Q>,
    Q: Ord,
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

#[cfg(feature = "alloc")]
impl<A> IntoIterator for ArrayMap<A>
where
    A: MapArray,
{
    type Item = (A::Key, A::Value);
    type IntoIter = IntoIter<A>;
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
pub struct IntoIter<A>
where
    A: MapArray,
{
    iter: std::vec::IntoIter<Entry<(A::Key, A::Value)>>,
}

#[cfg(feature = "alloc")]
impl<A> Iterator for IntoIter<A>
where
    A: MapArray,
{
    type Item = (A::Key, A::Value);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|entry| unsafe { entry.assume_init() })
    }
}

/// An iterator over references to the key-value pairs in an ArrayMap
pub struct Iter<'a, A>
where
    A: MapArray,
{
    iter: core::slice::Iter<'a, Entry<(A::Key, A::Value)>>,
}

impl<'a, A> Iterator for Iter<'a, A>
where
    A: MapArray,
{
    type Item = (&'a A::Key, &'a A::Value);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|entry| {
            let pair = unsafe { entry.as_ptr().as_ref() }.unwrap();
            (&pair.0, &pair.1)
        })
    }
}

/// An iterator over references to keys and mutable references to values in an ArrayMap
pub struct IterMut<'a, A>
where
    A: MapArray,
{
    iter: core::slice::IterMut<'a, Entry<(A::Key, A::Value)>>,
}

impl<'a, A> Iterator for IterMut<'a, A>
where
    A: MapArray,
    A::Key: 'a,
    A::Value: 'a,
{
    type Item = (&'a A::Key, &'a mut A::Value);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|entry| {
            let pair = unsafe { entry.as_mut_ptr().as_mut() }.unwrap();
            (&pair.0, &mut pair.1)
        })
    }
}

/// An iterator over references to the keys in an ArrayMap
pub struct Keys<'a, A>
where
    A: MapArray,
{
    iter: core::slice::Iter<'a, Entry<(A::Key, A::Value)>>,
}

impl<'a, A> Iterator for Keys<'a, A>
where
    A: MapArray,
    A::Key: 'a,
{
    type Item = &'a A::Key;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|entry| &unsafe { entry.as_ptr().as_ref() }.unwrap().0)
    }
}

/// An iterator over references to the values in an ArrayMap
pub struct Values<'a, A>
where
    A: MapArray,
{
    iter: core::slice::Iter<'a, Entry<(A::Key, A::Value)>>,
}

impl<'a, A> Iterator for Values<'a, A>
where
    A: MapArray,
    A::Value: 'a,
{
    type Item = &'a A::Value;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|entry| &unsafe { entry.as_ptr().as_ref() }.unwrap().1)
    }
}

/// An iterator over mutable references to the values in an ArrayMap
pub struct ValuesMut<'a, A>
where
    A: MapArray,
{
    iter: core::slice::IterMut<'a, Entry<(A::Key, A::Value)>>,
}

impl<'a, A> Iterator for ValuesMut<'a, A>
where
    A: MapArray,
    A::Value: 'a,
{
    type Item = &'a mut A::Value;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|entry| &mut unsafe { entry.as_mut_ptr().as_mut() }.unwrap().1)
    }
}
