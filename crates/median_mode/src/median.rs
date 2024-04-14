mod midpoint;
use midpoint::Midpoint;

pub trait Median {
    type Output;

    fn median_in_place(&mut self) -> Option<Self::Output>;
    fn median(&self) -> Option<Self::Output>;
}

impl<T> Median for [T]
where
    T: Copy + Midpoint + Ord,
{
    type Output = T;

    fn median_in_place(&mut self) -> Option<Self::Output> {
        match self.len() {
            0 => None,
            len if len % 2 == 0 => {
                self.sort_unstable();

                let middle = len / 2;
                Some(Midpoint::midpoint(self[middle - 1], self[middle]))
            }
            len => {
                self.sort_unstable();
                Some(self[len / 2])
            }
        }
    }

    fn median(&self) -> Option<Self::Output> {
        self.to_vec().median_in_place()
    }
}
