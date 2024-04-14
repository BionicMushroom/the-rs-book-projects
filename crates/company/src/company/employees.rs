use std::collections::{btree_set, BTreeSet};
use std::iter::FusedIterator;

pub(in crate::company) type Set = BTreeSet<String>;
pub(in crate::company) type SetIter<'a> = btree_set::Iter<'a, String>;

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug, Default)]
pub struct Iter<'a> {
    iter_opt: Option<SetIter<'a>>,
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter_opt
            .as_mut()
            .and_then(DoubleEndedIterator::next_back)
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.iter_opt.as_ref().map_or(0, ExactSizeIterator::len)
    }
}

impl FusedIterator for Iter<'_> {}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_opt.as_mut().and_then(Iterator::next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter_opt
            .as_ref()
            .map_or((0, Some(0)), Iterator::size_hint)
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
    pub(in crate::company) fn from_set_iter(iter: SetIter) -> Iter {
        Iter {
            iter_opt: Some(iter),
        }
    }
}
