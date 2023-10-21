use crate::{Slice, Sliceable, VecSlice};

pub struct Drain<'slice, 'borrow, T> {
    original: &'borrow mut VecSlice<'slice, T>,
    start: usize,
    elements: usize,
}

impl<'slice, 'borrow, T> Drain<'slice, 'borrow, T> {
    /// Ensures that the range is valid.
    pub(crate) fn new(
        original: &'borrow mut VecSlice<'slice, T>,
        range: impl core::ops::RangeBounds<usize>,
    ) -> Self {
        let (start, end) = VecSlice::<T>::translate_range(range, 0, original.len());
        assert!(start <= end && end <= original.len(), "range out of bounds");
        Self {
            original,
            start,
            elements: end - start,
        }
    }
}

impl<T> Iterator for Drain<'_, '_, T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        debug_assert!(self.start + self.elements <= self.original.len());
        if self.elements > 0 {
            self.elements -= 1;
            Some(self.original.remove(self.start))
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for Drain<'_, '_, T> {
    fn len(&self) -> usize {
        self.elements
    }
}

impl<T> Drop for Drain<'_, '_, T> {
    fn drop(&mut self) {
        self.for_each(|_| {});
    }
}
