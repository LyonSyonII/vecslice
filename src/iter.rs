use crate::{Slice, VecSlice};

impl<'a, T, S> IntoIterator for &'a VecSlice<'_, T, S>
where
    S: Slice<T> + ?Sized,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T, S> IntoIterator for &'a mut VecSlice<'_, T, S>
where
    S: Slice<T> + ?Sized,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}

impl<'a, T, S> From<&'a mut S> for VecSlice<'a, T, S>
where
    S: Slice<T> + ?Sized,
{
    fn from(original: &'a mut S) -> Self {
        Self::new(.., original)
    }
}
