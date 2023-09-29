struct VecSlice<'a, T, S> where S: Slice<T> + ?Sized {
    start: usize,
    end: usize,
    original: &'a mut S,
    _type: core::marker::PhantomData<T>,
}

impl<'a, T, S> VecSlice<'a, T, S> where S: Slice<T> + ?Sized {
    fn translate_range(range: impl core::ops::RangeBounds<usize>, start: usize, end: usize) -> (usize, usize) {
        use core::ops::Bound::*;
        match (range.start_bound(), range.end_bound()) {
            (Included(&s), Included(&e)) => (start+s, start+e+1),
            (Included(&s), Excluded(&e)) => (start+s, start+e),
            (Included(&s), Unbounded) => (start+s, end),
            (Excluded(&s), Included(&e)) => (start+s+1, start+e+1),
            (Excluded(&s), Excluded(&e)) => (start+s+1, start+e),
            (Excluded(&s), Unbounded) => (start+s+1, end),
            (Unbounded, Included(&e)) => (start, start+e+1),
            (Unbounded, Excluded(&e)) => (start, start+e),
            (Unbounded, Unbounded) => (start, end),
        }
    }

    pub fn new(range: impl core::ops::RangeBounds<usize>, original: &'a mut S) -> VecSlice<'a, T, S> {
        let (start, end) = VecSlice::<T, S>::translate_range(range, 0, original.len());
        VecSlice { start, end, original, _type: core::marker::PhantomData }
    }
}

trait Slice<T> {
    fn vecslice(&mut self, range: impl core::ops::RangeBounds<usize>) -> VecSlice<'_, T, Self>;
    
    fn insert(&mut self, index: usize, element: T);
    
    fn remove(&mut self, index: usize) -> T;
    
    fn len(&self) -> usize;
}

impl<T> Slice<T> for Vec<T> {
    fn vecslice(&mut self, range: impl core::ops::RangeBounds<usize>) -> VecSlice<'_, T, Self> {
        VecSlice::new(range, self)
    }
    
    fn insert(&mut self, index: usize, element: T) {
        self.insert(index, element)
    }

    fn remove(&mut self, index: usize) -> T {
        self.remove(index)
    }

    fn len(&self) -> usize {
        self.len()
    }
}