use crate::VecSlice;

impl<T> core::ops::Index<usize> for VecSlice<'_, T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[self.start+index]
    }
}

impl<T> core::ops::IndexMut<usize> for VecSlice<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[self.start+index]
    }
}

impl<T> core::ops::Index<core::ops::Range<usize>> for VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::Range<usize>> for VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::Range<usize>) -> &mut Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeFrom<usize>> for VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeFrom<usize>) -> &Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeFrom<usize>> for VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::RangeFrom<usize>) -> &mut Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeFull> for VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeFull) -> &Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeFull> for VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::RangeFull) -> &mut Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeInclusive<usize>> for VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeInclusive<usize>) -> &Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeInclusive<usize>> for VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::RangeInclusive<usize>) -> &mut Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeTo<usize>> for VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeTo<usize>) -> &Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeTo<usize>> for VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::RangeTo<usize>) -> &mut Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeToInclusive<usize>> for VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeToInclusive<usize>) -> &Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeToInclusive<usize>> for VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::RangeToInclusive<usize>) -> &mut Self::Output {
        let (start, end) = Self::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

// ---

impl<T> core::ops::Index<usize> for &VecSlice<'_, T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[self.start+index]
    }
}

impl<T> core::ops::Index<core::ops::Range<usize>> for &VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeFrom<usize>> for &VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeFrom<usize>) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeFull> for &VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeFull) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeInclusive<usize>> for &VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeInclusive<usize>) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeTo<usize>> for &VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeTo<usize>) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeToInclusive<usize>> for &VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeToInclusive<usize>) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

// ---

impl<T> core::ops::Index<usize> for &mut VecSlice<'_, T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[self.start+index]
    }
}

impl<T> core::ops::IndexMut<usize> for &mut VecSlice<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[self.start+index]
    }
}

impl<T> core::ops::Index<core::ops::Range<usize>> for &mut VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::Range<usize>> for &mut VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::Range<usize>) -> &mut Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeFrom<usize>> for &mut VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeFrom<usize>) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeFrom<usize>> for &mut VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::RangeFrom<usize>) -> &mut Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeFull> for &mut VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeFull) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeFull> for &mut VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::RangeFull) -> &mut Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeInclusive<usize>> for &mut VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeInclusive<usize>) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeInclusive<usize>> for &mut VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::RangeInclusive<usize>) -> &mut Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeTo<usize>> for &mut VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeTo<usize>) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeTo<usize>> for &mut VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::RangeTo<usize>) -> &mut Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}

impl<T> core::ops::Index<core::ops::RangeToInclusive<usize>> for &mut VecSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: core::ops::RangeToInclusive<usize>) -> &Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &self.vec[start..end]
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeToInclusive<usize>> for &mut VecSlice<'_, T> {
    fn index_mut(&mut self, index: core::ops::RangeToInclusive<usize>) -> &mut Self::Output {
        let (start, end) = VecSlice::<T>::translate_range(index, self.start, self.end);
        &mut self.vec[start..end]
    }
}