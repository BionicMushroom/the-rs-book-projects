use crate::SortedSlice;
use std::{cmp::Ordering, collections::HashMap, hash::Hash};

pub trait Mode {
    type Output;
    fn mode(&self) -> Option<Vec<Self::Output>>;
}

impl<T> Mode for SortedSlice<'_, T>
where
    T: Copy + PartialEq,
{
    type Output = T;

    fn mode(&self) -> Option<Vec<Self::Output>> {
        match self.slice.len() {
            0 => None,
            1 => Some(vec![*self.slice.first().unwrap()]),
            _ => {
                let mut modes = Vec::new();
                let mut curr_run_count = 1usize;
                let mut max_run_count = 1usize;

                for window in self.slice.windows(2) {
                    let (prev_elem, curr_elem) = (window[0], window[1]);

                    if prev_elem == curr_elem {
                        curr_run_count += 1;
                    } else {
                        match curr_run_count.cmp(&max_run_count) {
                            Ordering::Greater => {
                                max_run_count = curr_run_count;

                                modes.clear();
                                modes.push(prev_elem);
                            }
                            Ordering::Equal => modes.push(prev_elem),
                            Ordering::Less => (),
                        }

                        curr_run_count = 1;
                    }
                }

                match curr_run_count.cmp(&max_run_count) {
                    Ordering::Greater => {
                        modes.clear();
                        modes.push(*self.slice.last().unwrap());
                    }
                    Ordering::Equal => modes.push(*self.slice.last().unwrap()),
                    Ordering::Less => (),
                }

                Some(modes)
            }
        }
    }
}

impl<T> Mode for [T]
where
    T: Copy + Eq + Hash,
{
    type Output = T;

    fn mode(&self) -> Option<Vec<Self::Output>> {
        if self.is_empty() {
            None
        } else {
            let mut modes = Vec::new();
            let mut occurrences = HashMap::new();

            let mut max_occurrences = 0usize;

            for item in self {
                let curr_occurrences = occurrences.entry(item).or_insert(0usize);
                *curr_occurrences += 1;

                match (*curr_occurrences).cmp(&max_occurrences) {
                    Ordering::Greater => {
                        modes.clear();
                        modes.push(*item);

                        max_occurrences = *curr_occurrences;
                    }
                    Ordering::Equal => modes.push(*item),
                    Ordering::Less => (),
                }
            }

            Some(modes)
        }
    }
}
