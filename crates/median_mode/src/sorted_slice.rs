pub struct SortedSlice<'a, T> {
    pub slice: &'a [T],
}

impl<T> SortedSlice<'_, T> {
    pub fn from_sorted(slice: &[T]) -> SortedSlice<T> {
        SortedSlice { slice }
    }

    pub fn from_unsorted(slice: &mut [T]) -> SortedSlice<T>
    where
        T: Ord,
    {
        slice.sort_unstable();
        SortedSlice::from_sorted(slice)
    }
}
