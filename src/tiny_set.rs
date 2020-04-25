//! A set that starts on the stack but can automatically move to the heap

use core::{borrow::Borrow, fmt, iter::FromIterator, mem::swap};
use std::collections::BTreeSet;

use crate::{Array, ArraySet};

/**
A set that starts on the stack but can automatically move to the heap
*/
pub enum TinySet<A>
where
    A: Array,
{
    /// A set with items on the stack
    Stack(ArraySet<A>),
    /// A set with items on the heap
    Heap(BTreeSet<A::Item>),
}

impl<A> Default for TinySet<A>
where
    A: Array,
{
    fn default() -> Self {
        TinySet::Stack(ArraySet::default())
    }
}

impl<A> Clone for TinySet<A>
where
    A: Array,
    A::Item: Clone,
{
    fn clone(&self) -> Self {
        match self {
            TinySet::Stack(set) => TinySet::Stack(set.clone()),
            TinySet::Heap(set) => TinySet::Heap(set.clone()),
        }
    }
}

impl<A> TinySet<A>
where
    A: Array,
{
    /**
    Creates a new empty TinySet

    # Example
    ```
    use tinymap::*;

    let mut set = TinySet::<[Entry<i32>; 2]>::new();
    ```
    */
    pub fn new() -> Self {
        Self::default()
    }
    /**
    Clears the set, removing all elements

    # Example
    ```
    use tinymap::*;

    let mut v = TinySet::<[Entry<i32>; 2]>::new();
    v.insert(1);
    v.clear();
    assert!(v.is_empty());
    ```
    */
    pub fn clear(&mut self) {
        *self = Self::new();
    }
    /**
    Returns the number of elements in the set

    # Example

    ```
    use tinymap::*;

    let mut v = TinySet::<[Entry<i32>; 2]>::new();
    assert_eq!(v.len(), 0);
    v.insert(1);
    assert_eq!(v.len(), 1);
    ```
    */
    pub fn len(&self) -> usize {
        match self {
            TinySet::Stack(set) => set.len(),
            TinySet::Heap(set) => set.len(),
        }
    }
    /**
    Returns `true` if the set contains no elements

    # Example

    ```
    use tinymap::*;

    let mut v = TinySet::<[Entry<i32>; 2]>::new();
    assert!(v.is_empty());
    v.insert(1);
    assert!(!v.is_empty());
    ```
    */
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /**
    Returns the maximum number of elements the set can contain

    # Example

    ```
    use tinymap::*;

    let mut a = TinySet::<[Entry<i32>; 2]>::new();
    assert_eq!(2, a.capacity());
    ```
    */
    pub fn capacity(&self) -> usize {
        A::CAPACITY
    }
    /**
    Gets an iterator over the entries of the set, sorted

    # Example

    ```
    use tinymap::*;

    let set: TinySet<[Entry<i32>; 3]> = [3, 1, 2].iter().copied().collect();
    let mut set_iter = set.iter();
    assert_eq!(set_iter.next(), Some(&1));
    assert_eq!(set_iter.next(), Some(&2));
    assert_eq!(set_iter.next(), Some(&3));
    assert_eq!(set_iter.next(), None);
    ```
    */
    pub fn iter(&self) -> Iter<'_, A::Item> {
        match self {
            TinySet::Stack(set) => Iter::Stack(set.iter()),
            TinySet::Heap(set) => Iter::Heap(set.iter()),
        }
    }
}

impl<A> TinySet<A>
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
    use tinymap::*;

    let mut set = TinySet::<[Entry<i32>; 2]>::new();

    assert_eq!(set.insert(2), true);
    assert_eq!(set.insert(2), false);
    assert_eq!(set.len(), 1);
    ```
    */
    pub fn insert(&mut self, value: A::Item) -> bool {
        match self {
            TinySet::Stack(set) => match set.try_insert(value) {
                Ok(res) => res,
                Err(val) => {
                    let mut replacement_set = ArraySet::default();
                    swap(&mut replacement_set, set);
                    let mut btree_set = BTreeSet::new();
                    for val in replacement_set.into_iter() {
                        btree_set.insert(val);
                    }
                    let res = btree_set.insert(val);
                    *self = TinySet::Heap(btree_set);
                    res
                }
            },
            TinySet::Heap(set) => set.insert(value),
        }
    }
    /**
    Returns true if the set contains a value for the specified value

    # Example

    ```
    use tinymap::*;

    let set: TinySet<[Entry<_>; 3]> = [1, 2, 3].iter().copied().collect();
    assert_eq!(set.contains(&1), true);
    assert_eq!(set.contains(&4), false);
    ```
    */
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        A::Item: Borrow<Q>,
        Q: Ord,
    {
        match self {
            TinySet::Stack(set) => set.contains(value),
            TinySet::Heap(set) => set.contains(value),
        }
    }
    /**
    Returns a reference to the value if it is present

    # Example

    ```
    use tinymap::*;

    let mut set: TinySet<[Entry<_>; 3]> = [1, 2, 3].iter().copied().collect();
    assert_eq!(set.get(&2), Some(&2));
    assert_eq!(set.get(&4), None);
    ```
    */
    pub fn get<Q>(&self, value: &Q) -> Option<&A::Item>
    where
        A::Item: Borrow<Q>,
        Q: Ord,
    {
        match self {
            TinySet::Stack(set) => set.get(value),
            TinySet::Heap(set) => set.get(value),
        }
    }
    /**
    Removes a value from the set. Returns whether the value was present in the set.

    # Example

    ```
    use tinymap::*;

    let mut set = TinySet::<[Entry<i32>; 2]>::new();

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
        match self {
            TinySet::Stack(set) => set.remove(value),
            TinySet::Heap(set) => set.remove(value),
        }
    }
}

impl<A> fmt::Debug for TinySet<A>
where
    A: Array,
    A::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<A> From<ArraySet<A>> for TinySet<A>
where
    A: Array,
{
    fn from(set: ArraySet<A>) -> Self {
        TinySet::Stack(set)
    }
}

impl<A> From<A> for TinySet<A>
where
    A: Array,
    A::Item: Ord,
{
    fn from(array: A) -> Self {
        TinySet::from(ArraySet::from(array))
    }
}

impl<A> From<BTreeSet<A::Item>> for TinySet<A>
where
    A: Array,
{
    fn from(set: BTreeSet<A::Item>) -> Self {
        TinySet::Heap(set)
    }
}

impl<A> PartialEq for TinySet<A>
where
    A: Array,
    A::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<A> Eq for TinySet<A>
where
    A: Array,
    A::Item: Eq,
{
}

impl<'a, A> IntoIterator for &'a TinySet<A>
where
    A: Array,
{
    type Item = &'a A::Item;
    type IntoIter = Iter<'a, A::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(feature = "alloc")]
impl<A> IntoIterator for TinySet<A>
where
    A: Array,
{
    type Item = A::Item;
    type IntoIter = IntoIter<A::Item>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            TinySet::Stack(set) => IntoIter::Stack(set.into_iter()),
            TinySet::Heap(set) => IntoIter::Heap(set.into_iter()),
        }
    }
}

/// Elements from the iterator beyond the set's capacity will be discarded.
impl<A> FromIterator<A::Item> for TinySet<A>
where
    A: Array,
    A::Item: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = A::Item>,
    {
        let mut set = TinySet::default();
        for value in iter.into_iter().take(A::CAPACITY) {
            set.insert(value);
        }
        set
    }
}

/// An consuming iterator over the values in an ArraySet
pub enum IntoIter<T> {
    #[doc(hidden)]
    Stack(crate::array_set::IntoIter<T>),
    #[doc(hidden)]
    Heap(std::collections::btree_set::IntoIter<T>),
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IntoIter::Stack(iter) => iter.next(),
            IntoIter::Heap(iter) => iter.next(),
        }
    }
}

/// An iterator over references to the values in an TinySet
pub enum Iter<'a, T> {
    #[doc(hidden)]
    Stack(crate::array_set::Iter<'a, T>),
    #[doc(hidden)]
    Heap(std::collections::btree_set::Iter<'a, T>),
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Stack(iter) => iter.next(),
            Iter::Heap(iter) => iter.next(),
        }
    }
}
