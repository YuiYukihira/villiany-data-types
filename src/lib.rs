use std::{collections::HashMap, convert::TryFrom, fmt, marker::PhantomData, ops::{Add, Div, Index, IndexMut, Mul, Rem}, str::FromStr, usize};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::{Error, SeqAccess, Unexpected, Visitor, value::U8Deserializer}, ser::SerializeTuple};

pub type BlockID = usize;

pub trait DeSerializable<'de>: Sized {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
}

impl<'de, T: Serialize + Deserialize<'de> + Default + Copy, const N: usize> DeSerializable<'de>
    for [T; N]
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(N)?;
        for elem in &self[..] {
            seq.serialize_element(elem)?;
        }
        seq.end()
    }

    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ArrayVisitor<T, const N: usize>(PhantomData<[T; N]>);

        impl<'de, T: Deserialize<'de> + Default + Copy, const N: usize> Visitor<'de>
            for ArrayVisitor<T, N>
        {
            type Value = [T; N];

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an array of length {}", N)
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<[T; N], A::Error> {
                let mut arr = [T::default(); N];
                for (i, elem) in arr.iter_mut().enumerate() {
                    *elem = seq
                        .next_element()?
                        .ok_or_else(|| A::Error::invalid_length(i, &self))?;
                }
                Ok(arr)
            }
        }

        let visitor: ArrayVisitor<T, N> = ArrayVisitor(PhantomData);
        deserializer.deserialize_tuple(N, visitor)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct ChunkDataRow<const N: usize>(
    #[serde(with = "DeSerializable")]
    [BlockID; N]
);

impl<const N: usize> ChunkDataRow<N> {
    fn new(block: BlockID) -> Self {
        Self([block; N])
    }
}

impl<const N: usize> Default for ChunkDataRow<N> {
    fn default() -> Self {
        ChunkDataRow([BlockID::default(); N])
    }
}

impl<const N: usize> Index<usize> for ChunkDataRow<N> {
    type Output = BlockID;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for ChunkDataRow<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct ChunkData<const X: usize, const Y: usize>(
    #[serde(with = "DeSerializable")]
    [ChunkDataRow<X>; Y],
);

impl<const X: usize, const Y: usize> ChunkData<X, Y> {
    pub fn new(block: BlockID) -> Self {
        Self([ChunkDataRow::new(block); Y])
    }
}

impl<const X: usize, const Y: usize> Default for ChunkData<X, Y> {
    fn default() -> Self {
        Self([ChunkDataRow::default(); Y])
    }
}

impl<const X: usize, const Y: usize> Index<usize> for ChunkData<X, Y> {
    type Output = ChunkDataRow<X>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const X: usize, const Y: usize> IndexMut<usize> for ChunkData<X, Y> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Coord(isize, isize);

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> isize {
        self.0
    }

    pub fn y(&self) -> isize {
        self.1
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl Add<Coord> for Coord {
    type Output = Self;
    fn add(self, rhs: Coord) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<Coord> for Coord {
    type Output = Self;
    fn mul(self, rhs: Coord) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div<Coord> for Coord {
    type Output = Self;
    fn div(self, rhs: Coord) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl Div<isize> for Coord {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Rem<Coord> for Coord {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0, self.1 % rhs.1)
    }
}