use crate::{Slice, VecSlice};

impl<'a, T, S> IntoIterator for &'a VecSlice<'_, T, S>
where
    S: Slice<T>,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}

impl<'a, T, S> IntoIterator for &'a mut VecSlice<'_, T, S>
where
    S: Slice<T>,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().iter_mut()
    }
}
