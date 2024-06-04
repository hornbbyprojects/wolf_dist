use std::ops::Add;

pub struct AddedTuple<T, U>(pub T, pub U);

impl<T: Add<Output = T>, U: Add<Output = U>> AddedTuple<T, U> {
    pub fn combine_result(self, other: Self) -> Self {
        AddedTuple(self.0 + other.0, self.1 + other.1)
    }
}

pub struct CantCombine<T>(pub T);
impl<T> CantCombine<T> {
    pub fn combine_result(self, _other: Self) -> Self {
        self
    }
    pub fn extract(self) -> T {
        self.0
    }
}
