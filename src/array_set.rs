//! An array-backed, set-like data structure

use core::{borrow::Borrow, fmt, iter::FromIterator, mem::swap};

use crate::Array;

/**
An array-backed, set-like data structure

ArraySet wraps an array of values and supports operation similar to a BTreeSet or HashSet.
It has a fixed capacity, but it keeps track of how many values have been inserted and removed.

Because this crate uses no unsafe code, value types must implement Default.
*/
#[derive(Clone, Copy, Default)]
pub struct ArraySet<A> {
    array: A,
    len: usize,
}

impl<A> ArraySet<A>
where
    A: Default,
{
    /**
    Creates a new empty ArraySet

    # Example
    ```
    use tinymap::ArraySet;

    let mut set = ArraySet::<[i32; 10]>::new();
    ```
    */
    pub fn new() -> Self {
        Self::default()
    }
    /**
    Clears the set, removing all elements

    # Example
    ```
    use tinymap::ArraySet;

    let mut v = ArraySet::<[i32; 10]>::new();
    v.insert(1);
    v.clear();
    assert!(v.is_empty());
    ```
    */
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl<A> ArraySet<A> {
    /**
    Returns the number of elements in the set

    # Example

    ```
    use tinymap::ArraySet;

    let mut v = ArraySet::<[i32; 10]>::new();
    assert_eq!(v.len(), 0);
    v.insert(1);
    assert_eq!(v.len(), 1);
    ```
    */
    pub const fn len(&self) -> usize {
        self.len
    }
    /**
    Returns `true` if the set contains no elements

    # Example

    ```
    use tinymap::ArraySet;

    let mut v = ArraySet::<[i32; 10]>::new();
    assert!(v.is_empty());
    v.insert(1);
    assert!(!v.is_empty());
    ```
    */
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<A> ArraySet<A>
where
    A: Array,
{
    /**
    Returns the maximum number of elements the set can contain

    # Example

    ```
    use tinymap::ArraySet;

    let mut a = ArraySet::<[i32; 10]>::new();
    assert_eq!(10, a.capacity());
    ```
    */
    pub fn capacity(&self) -> usize {
        A::CAPACITY
    }
    /**
    Gets an iterator over the entries of the set, sorted

    # Example

    ```
    use tinymap::ArraySet;

    let set = ArraySet::from([3, 1, 2]);
    let mut set_iter = set.iter();
    assert_eq!(set_iter.next(), Some(&1));
    assert_eq!(set_iter.next(), Some(&2));
    assert_eq!(set_iter.next(), Some(&3));
    assert_eq!(set_iter.next(), None);
    ```
    */
    pub fn iter(&self) -> Iter<'_, A> {
        Iter {
            iter: self.array.as_slice()[..self.len].iter(),
        }
    }
}

impl<A> ArraySet<A>
where
    A: Array,
    A::Item: Ord,
{
    /**
    Inserts an value into the set

    If the set did not have this value present, true is returned.

    If the set did have this value present, `false` is returned, and the entry is not updated.

    # Panics

    Panics if insertion would cause the set to excede its capacity.

    # Example

    ```
    use tinymap::ArraySet;

    let mut set = ArraySet::<[i32; 10]>::new();

    assert_eq!(set.insert(2), true);
    assert_eq!(set.insert(2), false);
    assert_eq!(set.len(), 1);
    ```
    */
    pub fn insert(&mut self, value: A::Item) -> bool {
        self.try_insert(value)
            .unwrap_or_else(|_| panic!("Insertion would excede capacity"))
    }
    /**
    Attempts to insert a value into the set

    If the set did not have this value present, true is returned.

    If the set did have this value present, `false` is returned, and the entry is not updated.

    # Errors

    If insertion would cause the set to excede its capacity, this function returns an error containing
    the value that could not be inserted.

    # Example

    ```
    use tinymap::ArraySet;

    let mut set = ArraySet::<[i32; 3]>::new();
    assert!(set.try_insert(37).is_ok());
    assert!(set.try_insert(2).is_ok());
    assert!(set.try_insert(16).is_ok());
    assert!(set.try_insert(0).is_err());
    ```
    */
    pub fn try_insert(&mut self, mut value: A::Item) -> Result<bool, A::Item> {
        if self.len == A::CAPACITY {
            return Err(value);
        }
        match self.find(&value) {
            Ok(_) => Ok(false),
            Err(i) => {
                let slice = self.array.as_mut_slice();
                for j in ((i + 1)..=self.len).rev() {
                    slice.swap(j - 1, j);
                }
                swap(&mut value, &mut slice[i]);
                self.len += 1;
                Ok(true)
            }
        }
    }
}

impl<A> ArraySet<A>
where
    A: Array,
{
    fn find<Q>(&self, value: &Q) -> Result<usize, usize>
    where
        A::Item: Borrow<Q>,
        Q: Ord,
    {
        self.array.as_slice()[..self.len]
            .binary_search_by_key(&value.borrow(), |value| value.borrow())
    }
    /**
    Returns true if the set contains a value for the specified value

    # Example

    ```
    use tinymap::ArraySet;

    let set = ArraySet::from([1, 2, 3]);
    assert_eq!(set.contains(&1), true);
    assert_eq!(set.contains(&4), false);
    ```
    */
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        A::Item: Borrow<Q>,
        Q: Ord,
    {
        self.find(value).is_ok()
    }
    /**
    Returns a reference to the value corresponding to the value

    # Example

    ```
    use tinymap::ArraySet;

    let mut set = ArraySet::from([1, 2, 3]);
    assert_eq!(set.get(&2), Some(&2));
    assert_eq!(set.get(&4), None);
    ```
    */
    pub fn get<Q>(&self, value: &Q) -> Option<&A::Item>
    where
        A::Item: Borrow<Q>,
        Q: Ord,
    {
        if let Ok(i) = self.find(value) {
            Some(&self.array.as_slice()[i])
        } else {
            None
        }
    }
}

impl<A> ArraySet<A>
where
    A: Array,
    A::Item: Ord + Default,
{
    /**
    Removes a value from the set. Returns whether the value was present in the set.

    # Example

    ```
    use tinymap::ArraySet;

    let mut set = ArraySet::<[i32; 10]>::new();

    set.insert(2);
    assert_eq!(set.remove(&2), true);
    assert_eq!(set.remove(&2), false);
    ```
    */
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        A::Item: Borrow<Q>,
        Q: Ord,
    {
        if let Ok(i) = self.find(value) {
            let slice = self.array.as_mut_slice();
            let mut value = A::Item::default();
            swap(&mut value, &mut slice[i]);
            for j in (i + 1)..self.len {
                slice.swap(j - 1, j);
            }
            self.len -= 1;
            true
        } else {
            false
        }
    }
}

impl<A> fmt::Debug for ArraySet<A>
where
    A: Array,
    A::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<A> From<A> for ArraySet<A>
where
    A: Array,
    A::Item: Ord,
{
    fn from(mut array: A) -> Self {
        array.as_mut_slice().sort_unstable();
        ArraySet {
            array,
            len: A::CAPACITY,
        }
    }
}

#[cfg(feature = "alloc")]
impl<A> IntoIterator for ArraySet<A>
where
    A: Array,
{
    type Item = A::Item;
    type IntoIter = IntoIter<A>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.array.into_boxed_slice().into_vec().into_iter(),
        }
    }
}

/// Elements from the iterator beyond the set's capacity will be discarded.
impl<A> FromIterator<A::Item> for ArraySet<A>
where
    A: Array + Default,
    A::Item: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = A::Item>,
    {
        let mut set = ArraySet::default();
        for value in iter.into_iter().take(A::CAPACITY) {
            set.insert(value);
        }
        set
    }
}

/// An consuming iterator over the values in an ArrayMap
#[cfg(feature = "alloc")]
pub struct IntoIter<A>
where
    A: Array,
{
    iter: std::vec::IntoIter<A::Item>,
}

#[cfg(feature = "alloc")]
impl<A> Iterator for IntoIter<A>
where
    A: Array,
{
    type Item = A::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// An iterator over references to the values in an ArraySet
pub struct Iter<'a, A>
where
    A: Array,
{
    iter: core::slice::Iter<'a, A::Item>,
}

impl<'a, A> Iterator for Iter<'a, A>
where
    A: Array,
{
    type Item = &'a A::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
