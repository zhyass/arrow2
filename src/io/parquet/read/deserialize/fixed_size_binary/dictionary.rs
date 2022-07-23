use std::collections::VecDeque;

use parquet2::page::{DictPage, FixedLenByteArrayPageDict};

use crate::{
    array::{Array, DictionaryArray, DictionaryKey, FixedSizeBinaryArray},
    bitmap::MutableBitmap,
    datatypes::DataType,
    error::Result,
    io::parquet::read::deserialize::nested_utils::{InitNested, NestedArrayIter, NestedState},
};

use super::super::dictionary::*;
use super::super::utils::MaybeNext;
use super::super::DataPages;

/// An iterator adapter over [`DataPages`] assumed to be encoded as parquet's dictionary-encoded binary representation
#[derive(Debug)]
pub struct DictIter<K, I>
where
    I: DataPages,
    K: DictionaryKey,
{
    iter: I,
    data_type: DataType,
    values: Dict,
    items: VecDeque<(Vec<K>, MutableBitmap)>,
    chunk_size: Option<usize>,
}

impl<K, I> DictIter<K, I>
where
    K: DictionaryKey,
    I: DataPages,
{
    pub fn new(iter: I, data_type: DataType, chunk_size: Option<usize>) -> Self {
        Self {
            iter,
            data_type,
            values: Dict::Empty,
            items: VecDeque::new(),
            chunk_size,
        }
    }
}

fn read_dict(data_type: DataType, dict: &dyn DictPage) -> Box<dyn Array> {
    let data_type = match data_type {
        DataType::Dictionary(_, values, _) => *values,
        _ => data_type,
    };
    let dict = dict
        .as_any()
        .downcast_ref::<FixedLenByteArrayPageDict>()
        .unwrap();
    let values = dict.values().to_vec();

    Box::new(FixedSizeBinaryArray::from_data(
        data_type,
        values.into(),
        None,
    ))
}

impl<K, I> Iterator for DictIter<K, I>
where
    I: DataPages,
    K: DictionaryKey,
{
    type Item = Result<DictionaryArray<K>>;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_state = next_dict(
            &mut self.iter,
            &mut self.items,
            &mut self.values,
            self.data_type.clone(),
            self.chunk_size,
            |dict| read_dict(self.data_type.clone(), dict),
        );
        match maybe_state {
            MaybeNext::Some(Ok(dict)) => Some(Ok(dict)),
            MaybeNext::Some(Err(e)) => Some(Err(e)),
            MaybeNext::None => None,
            MaybeNext::More => self.next(),
        }
    }
}

#[derive(Debug)]
pub struct NestedDictIter<K, I>
where
    I: DataPages,
    K: DictionaryKey,
{
    iter: I,
    init: Vec<InitNested>,
    data_type: DataType,
    values: Dict,
    items: VecDeque<(NestedState, (Vec<K>, MutableBitmap))>,
    chunk_size: Option<usize>,
}

impl<K, I> NestedDictIter<K, I>
where
    I: DataPages,
    K: DictionaryKey,
{
    pub fn new(
        iter: I,
        init: Vec<InitNested>,
        data_type: DataType,
        chunk_size: Option<usize>,
    ) -> Self {
        Self {
            iter,
            init,
            data_type,
            values: Dict::Empty,
            items: VecDeque::new(),
            chunk_size,
        }
    }
}

impl<K, I> Iterator for NestedDictIter<K, I>
where
    I: DataPages,
    K: DictionaryKey,
{
    type Item = Result<(NestedState, DictionaryArray<K>)>;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_state = nested_next_dict(
            &mut self.iter,
            &mut self.items,
            &self.init,
            &mut self.values,
            self.data_type.clone(),
            self.chunk_size,
            |dict| read_dict(self.data_type.clone(), dict),
        );
        match maybe_state {
            MaybeNext::Some(Ok(dict)) => Some(Ok(dict)),
            MaybeNext::Some(Err(e)) => Some(Err(e)),
            MaybeNext::None => None,
            MaybeNext::More => self.next(),
        }
    }
}

/// Converts [`DataPages`] to an [`Iterator`] of [`Array`]
pub fn iter_to_arrays_nested<'a, K, I>(
    iter: I,
    init: Vec<InitNested>,
    data_type: DataType,
    chunk_size: Option<usize>,
) -> NestedArrayIter<'a>
where
    I: 'a + DataPages,
    K: DictionaryKey,
{
    Box::new(
        NestedDictIter::<K, I>::new(iter, init, data_type, chunk_size).map(|result| {
            let (mut nested, array) = result?;
            let _ = nested.nested.pop().unwrap(); // the primitive
            Ok((nested, array.boxed()))
        }),
    )
}
