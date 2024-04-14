use crate::company::employees;

use std::collections::{btree_map, BTreeMap};
use std::iter::FusedIterator;

pub(in crate::company) type Map = BTreeMap<String, employees::Set>;
pub(in crate::company) type MapIter<'a> = btree_map::Iter<'a, String, employees::Set>;

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug, Default)]
pub struct Iter<'a> {
    iter: MapIter<'a>,
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(Iter::map_iter_item_to_self_iter_item)
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl FusedIterator for Iter<'_> {}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a String, employees::Iter<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Iter::map_iter_item_to_self_iter_item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn min(mut self) -> Option<Self::Item> {
        self.next()
    }

    fn max(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl Iter<'_> {
    pub(in crate::company) fn from_map_iter(iter: MapIter) -> Iter {
        Iter { iter }
    }

    fn map_iter_item_to_self_iter_item(
        iter_item: <MapIter as Iterator>::Item,
    ) -> <Iter as Iterator>::Item {
        (
            iter_item.0,
            employees::Iter::from_set_iter(iter_item.1.iter()),
        )
    }
}
