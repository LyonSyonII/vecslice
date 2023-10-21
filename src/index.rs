use crate::{Slice, VecSlice};

impl<Idx: core::ops::RangeBounds<usize>, T> core::ops::Index<Idx> for VecSlice<'_, T> {
    type Output = [T];

    fn index(&self, index: Idx) -> &Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        self.as_ref().index(start..end)
    }
}

impl<Idx: core::ops::RangeBounds<usize>, T> core::ops::IndexMut<Idx> for VecSlice<'_, T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        self.as_mut().index_mut(start..end)
    }
}
