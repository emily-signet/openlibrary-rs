use std::marker::PhantomData;

use serde::de::Visitor;
use serde_with::{de::DeserializeAsWrap, DeserializeAs};
use smallvec::SmallVec;

pub(crate) struct SmallVecShim<T, const N: usize> {
    _spooky: PhantomData<fn() -> SmallVec<[T; N]>>,
}

struct SmallVecShimVisitor<T, U, const N: usize> {
    _spooky: PhantomData<fn() -> SmallVec<[T; N]>>,
    _spookier: PhantomData<U>,
}

impl<T, U, const N: usize> SmallVecShimVisitor<T, U, N> {
    fn new() -> Self {
        SmallVecShimVisitor {
            _spooky: PhantomData,
            _spookier: PhantomData,
        }
    }
}

impl<'de, T, U, const N: usize> Visitor<'de> for SmallVecShimVisitor<T, U, N>
where
    U: DeserializeAs<'de, T>,
{
    type Value = SmallVec<[T; N]>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a hashmap/dictionary")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut out: SmallVec<[T; N]> = SmallVec::with_capacity(seq.size_hint().unwrap_or(0));

        while let Some(e) = seq.next_element::<DeserializeAsWrap<T, U>>()? {
            out.push(e.into_inner());
        }

        Ok(out)
    }
}

impl<'de, T, U, const N: usize> DeserializeAs<'de, SmallVec<[T; N]>> for SmallVecShim<U, N>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<SmallVec<[T; N]>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(SmallVecShimVisitor::<T, U, N>::new())
    }
}