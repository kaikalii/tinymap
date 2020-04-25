use core::{fmt, marker::PhantomData};

use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{Array, ArrayMap, ArraySet, MapArray};
#[cfg(feature = "alloc")]
use crate::{TinyMap, TinySet};

impl<A> Serialize for ArrayMap<A>
where
    A: MapArray,
    A::Key: Serialize,
    A::Value: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self)
    }
}

#[cfg(feature = "alloc")]
impl<A> Serialize for TinyMap<A>
where
    A: MapArray,
    A::Key: Serialize,
    A::Value: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self)
    }
}

impl<A> Serialize for ArraySet<A>
where
    A: Array,
    A::Item: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self)
    }
}

#[cfg(feature = "alloc")]
impl<A> Serialize for TinySet<A>
where
    A: Array,
    A::Item: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self)
    }
}

impl<'de, A> Deserialize<'de> for ArrayMap<A>
where
    A: MapArray,
    A::Key: Deserialize<'de> + Ord,
    A::Value: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThisVisitor<'de, A>(PhantomData<(&'de (), A)>);

        impl<'de, A> Visitor<'de> for ThisVisitor<'de, A>
        where
            A: MapArray,
            A::Key: Deserialize<'de> + Ord,
            A::Value: Deserialize<'de>,
        {
            type Value = ArrayMap<A>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a map with no more than {} items", A::CAPACITY)
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut array_map = Self::Value::new();
                while let Some((key, value)) = map.next_entry()? {
                    if array_map.try_insert(key, value).is_err() {
                        return Err(M::Error::invalid_length(A::CAPACITY + 1, &self));
                    }
                }
                Ok(array_map)
            }
        }

        deserializer.deserialize_map(ThisVisitor::<A>(PhantomData))
    }
}

#[cfg(feature = "alloc")]
impl<'de, A> Deserialize<'de> for TinyMap<A>
where
    A: MapArray,
    A::Key: Deserialize<'de> + Ord,
    A::Value: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThisVisitor<'de, A>(PhantomData<(&'de (), A)>);

        impl<'de, A> Visitor<'de> for ThisVisitor<'de, A>
        where
            A: MapArray,
            A::Key: Deserialize<'de> + Ord,
            A::Value: Deserialize<'de>,
        {
            type Value = TinyMap<A>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a map with no more than {} items", A::CAPACITY)
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut tiny_map = Self::Value::new();
                while let Some((key, value)) = map.next_entry()? {
                    tiny_map.insert(key, value);
                }
                Ok(tiny_map)
            }
        }

        deserializer.deserialize_map(ThisVisitor::<A>(PhantomData))
    }
}

impl<'de, A> Deserialize<'de> for ArraySet<A>
where
    A: Array,
    A::Item: Deserialize<'de> + Ord,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThisVisitor<'de, A>(PhantomData<(&'de (), A)>);

        impl<'de, A> Visitor<'de> for ThisVisitor<'de, A>
        where
            A: Array,
            A::Item: Deserialize<'de> + Ord,
        {
            type Value = ArraySet<A>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a map with no more than {} items", A::CAPACITY)
            }

            fn visit_seq<S>(self, mut set: S) -> Result<Self::Value, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let mut array_set = Self::Value::new();
                while let Some(item) = set.next_element()? {
                    array_set.insert(item);
                }
                Ok(array_set)
            }
        }

        deserializer.deserialize_seq(ThisVisitor::<A>(PhantomData))
    }
}

#[cfg(feature = "alloc")]
impl<'de, A> Deserialize<'de> for TinySet<A>
where
    A: Array,
    A::Item: Deserialize<'de> + Ord,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThisVisitor<'de, A>(PhantomData<(&'de (), A)>);

        impl<'de, A> Visitor<'de> for ThisVisitor<'de, A>
        where
            A: Array,
            A::Item: Deserialize<'de> + Ord,
        {
            type Value = TinySet<A>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a map with no more than {} items", A::CAPACITY)
            }

            fn visit_seq<S>(self, mut set: S) -> Result<Self::Value, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let mut tiny_set = Self::Value::new();
                while let Some(item) = set.next_element()? {
                    tiny_set.insert(item);
                }
                Ok(tiny_set)
            }
        }

        deserializer.deserialize_seq(ThisVisitor::<A>(PhantomData))
    }
}
