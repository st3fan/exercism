pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

struct EvensEnumerate<I> {
    iter: I,
}

impl<I> Iterator for EvensEnumerate<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let result = self.iter.next();
        let _ = self.iter.next(); // Simply ignore the next (uneven) iteration
        result
    }
}

trait EvensEnumerable {
    fn evens(self) -> EvensEnumerate<Self>
    where
        Self: Sized;
}

impl<I> EvensEnumerable for I
where
    I: Iterator,
    I: Sized,
{
    fn evens(self) -> EvensEnumerate<Self>
    where
        Self: Sized,
    {
        EvensEnumerate { iter: self }
    }
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate()
        .filter(|(i, _e)| i % 2 == 0)
        .map(|(_i, e)| e)
}

pub fn evens_through_specialized_iterator<T>(
    iter: impl Iterator<Item = T>,
) -> impl Iterator<Item = T> {
    iter.evens()
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
