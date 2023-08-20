use core::ops::RangeBounds;

pub mod iter;
pub mod index;

/// Growable mutable reference on a [`Vec`].
/// 
/// Due to requiring a mutable reference to the underlying buffer, only one [`VecSlice`] can exist at a time, ensuring memory safety.
/// 
/// # Complexity
/// 
/// All operations have O(n) complexity, as the slice can start and end anywhere on the original vector.
/// 
/// If [`VecSlice::new_at_tail`] is used, the complexity of push operations will be O(1).
/// 
/// # Examples
/// 
/// ```
/// use vecslice::Slice;
/// 
/// let mut v = vec![1, 2, 3];
/// let mut slice = v.vecslice(0..=1);
/// assert_eq!(slice.len(), 2);
/// assert_eq!(slice, [1, 2]);
/// 
/// slice.push(3);
/// assert_eq!(slice, [1, 2, 3]);
/// assert_eq!(v, [1, 2, 3, 3]);
/// ```
#[derive(Eq, Ord)]
pub struct VecSlice<'a, T> {
    start: usize,
    end: usize,
    vec: &'a mut Vec<T>
}
impl<'a, T> VecSlice<'a, T> {
    fn translate_range(range: impl RangeBounds<usize>, start: usize, end: usize) -> (usize, usize) {
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

    pub fn new(range: impl RangeBounds<usize>, vec: &'a mut Vec<T>) -> VecSlice<'a, T> {
        let (start, end) = VecSlice::<T>::translate_range(range, 0, vec.len());
        VecSlice { start, end, vec }
    }
    
    /// Creates a new [`VecSlice`] at the tail of the current one.
    /// 
    /// The new slice will be empty, and newly added elements will be appended to the end of the [`VecSlice`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![1, 2, 3];
    /// let mut slice = vec.vecslice(0..);
    /// assert_eq!(slice, [1, 2, 3]);
    /// 
    /// let mut slice2 = slice.new_at_tail();
    /// assert_eq!(slice2, []);
    /// slice2.push(4);
    /// assert_eq!(slice2, [4]);
    /// 
    /// assert_eq!(slice, [1, 2, 3]);
    /// assert_eq!(vec, [1, 2, 3, 4]);
    /// ```
    pub fn new_at_tail(&mut self) -> VecSlice<'_, T> {
        VecSlice::new(self.end.., self.vec)
    }
    
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Appends an element to the back of a collection.
    /// 
    /// If you'd like to push at the front of the collection, use [`VecSlice::push_front`] instead.
    ///
    /// [`VecSlice::push_front`]: crate::VecSlice::push_front
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![0, 1, 2, 3];
    /// let mut slice = vec.vecslice(1..=2);
    /// assert_eq!(slice, [1, 2]);
    /// slice.push(4);
    /// slice.push(5);
    /// assert_eq!(slice, [1, 2, 4, 5]);
    /// assert_eq!(vec, [0, 1, 2, 4, 5, 3]);
    /// ```
    pub fn push(&mut self, element: T) {
        self.vec.insert(self.end, element);
        self.end += 1;
    }

    /// Appends an element to the front of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![0, 1, 2, 3];
    /// let mut slice = vec.vecslice(1..=2);
    /// assert_eq!(slice, [1, 2]);
    /// slice.push_front(4);
    /// slice.push_front(5);
    /// assert_eq!(slice, [5, 4, 1, 2]);
    /// assert_eq!(vec, [0, 5, 4, 1, 2, 3]);
    /// ```
    pub fn push_front(&mut self, element: T) {
        self.vec.insert(self.start, element);
        self.end += 1;
    }
    
    /// Inserts an element at position `index` within the slice, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![0, 1, 2, 3];
    /// let mut slice = vec.vecslice(1..=2);
    /// assert_eq!(slice, [1, 2]);
    /// slice.insert(1, 4);
    /// assert_eq!(slice, [1, 4, 2]);
    /// slice.insert(3, 5);
    /// assert_eq!(slice, [1, 4, 2, 5]);
    /// assert_eq!(vec, [0, 1, 4, 2, 5, 3]);
    /// ```
    pub fn insert(&mut self, index: usize, element: T) {
        assert!(index <= self.len());
        self.vec.insert(self.start+index, element);
        self.end += 1;
    }
    
    /// Removes the last element from a VecSlice and returns it, or [`None`] if it
    /// is empty.
    ///
    /// If you'd like to pop the first element, use [`VecSlice::pop_front`] instead.
    ///
    /// [`VecSlice::pop_front`]: crate::VecSlice::pop_front
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3];
    /// assert_eq!(vec.pop(), Some(3));
    /// assert_eq!(vec, [1, 2]);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.end -= 1;
            Some(self.vec.remove(self.end-1))
        } else {
            None
        }
    }

    /// Removes the last element from a VecSlice and returns it, or [`None`] if it
    /// is empty.
    ///
    /// If you'd like to pop the first element, use [`VecSlice::pop_front`] instead.
    ///
    /// [`VecSlice::pop_front`]: crate::VecSlice::pop_front
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3];
    /// assert_eq!(vec.pop(), Some(3));
    /// assert_eq!(vec, [1, 2]);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.end -= 1;
            Some(self.vec.remove(self.start))
        } else {
            None
        }
    }
    
    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// Note: Because this shifts over the remaining elements, it has a
    /// worst-case performance of *O*(*n*).
    /// 
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut v = vec![1, 2, 3];
    /// let mut slice = v.vecslice(0..2);
    /// assert_eq!(slice.remove(1), 2);
    /// assert_eq!(slice, [1]);
    /// assert_eq!(v, [1, 3]);
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index <= self.len());
        self.end -= 1;
        self.vec.remove(self.start+index)
    }
    
    /// Clears the slice, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the underlying vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![1, 2, 3];
    /// let mut slice = vec.vecslice(1..);
    ///
    /// slice.clear();
    ///
    /// assert!(slice.is_empty());
    /// assert_eq!(vec, [1]);
    /// ```
    pub fn clear(&mut self) {
        self.drain(..);
    }
    
    /// Removes the specified range from the slice in bulk, returning all
    /// removed elements as an iterator. If the iterator is dropped before
    /// being fully consumed, it drops the remaining removed elements.
    ///
    /// The returned iterator keeps a mutable borrow on the vector to optimize
    /// its implementation.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point or if
    /// the end point is greater than the length of the vector.
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped (due to
    /// [`mem::forget`], for example), the vector may have lost and leaked
    /// elements arbitrarily, including elements outside the range.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![0, 1, 2, 3, 4, 5];
    /// let mut slice = vec.vecslice(2..);
    /// assert_eq!(slice, [2, 3, 4, 5]);
    /// let u: Vec<_> = slice.drain(1..=2).collect();
    /// assert_eq!(slice, [2, 5]);
    /// assert_eq!(u, &[3, 4]);
    ///
    /// assert_eq!(vec, &[0, 1, 2, 5]);
    /// 
    /// let mut slice = vec.vecslice(1..=2);
    /// assert_eq!(slice, [1, 2]);
    /// slice.drain(..);
    /// assert_eq!(slice, []);
    /// assert_eq!(vec, &[0, 5]);
    /// ```
    pub fn drain(&mut self, range: impl RangeBounds<usize>) -> std::vec::Drain<'_, T> {
        let (start, end) = Self::translate_range(range, self.start, self.end);
        dbg!((start, end));
        self.end -= end - start; // Adjust length of the new slice
        self.vec.drain(start..end)
    }
    
    /// Copies `self` into a new `Vec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut v = vec![1, 2, 3];
    /// let slice = v.vecslice(0..2);
    /// let s = slice.to_vec();
    /// assert_eq!(s, [1, 2]);
    /// assert_eq!(v, [1, 2, 3]);
    /// // Here, `s` and `v` can be modified independently.
    /// ```
    pub fn to_vec(&self) -> Vec<T> where T: Clone {
        self.vec[self.start..self.end].to_vec()
    }
    
    /// Consumes `self` into a new `Vec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![1, 2, 3];
    /// let slice = vec.vecslice(0..2);
    /// let s = slice.to_vec();
    /// assert_eq!(s, [1, 2]);
    /// assert_eq!(vec, [1, 2, 3]);
    /// // Here, `s` and `v` can be modified independently.
    /// ```
    pub fn into_vec(self) -> Vec<T> where T: Clone {
        self.vec[self.start..self.end].to_vec()
    }
    
    /// Returns a reference to the underlying slice.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![1, 2, 3];
    /// let slice = vec.vecslice(0..2);
    /// assert_eq!(slice, [1, 2]);
    /// assert_eq!(slice.as_slice(), [1, 2]);
    pub fn as_slice(&self) -> &[T] {
        &self.vec[self.start..self.end]
    }

    /// Returns a mutable reference to the underlying slice.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![1, 2, 3];
    /// let mut slice = vec.vecslice(0..2);
    /// assert_eq!(slice, [1, 2]);
    /// let mut mut_slice = slice.as_mut_slice();
    /// assert_eq!(mut_slice, [1, 2]);
    /// mut_slice[0] = 4;
    /// assert_eq!(mut_slice, [4, 2]);
    /// assert_eq!(slice, [4, 2]);
    /// assert_eq!(vec, [4, 2, 3]);
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.vec[self.start..self.end]
    }
    
    /// Sorts the slice.
    ///
    /// This sort is stable (i.e., does not reorder equal elements) and *O*(*n* \* log(*n*)) worst-case.
    ///
    /// When applicable, unstable sorting is preferred because it is generally faster than stable
    /// sorting and it doesn't allocate auxiliary memory.
    /// See [`sort_unstable`](slice::sort_unstable).
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![-5, 4, -3, 2, 1];
    /// let mut slice = vec.vecslice(1..=3); 
    ///
    /// slice.sort();
    /// assert_eq!(slice, [-3, 2, 4]);
    /// assert_eq!(vec, [-5, -3, 2, 4, 1]);
    /// ```
    pub fn sort(&mut self) where T: Ord {
        self.vec[self.start..self.end].sort();
    }
    
    /// Sorts the slice, but might not preserve the order of equal elements.
    ///
    /// This sort is unstable (i.e., may reorder equal elements), in-place
    /// (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.
    ///
    /// It is typically faster than stable sorting, except in a few special cases, e.g., when the
    /// slice consists of several concatenated sorted sequences.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    /// 
    /// let mut vec = vec![-5, 4, -3, 2, 1];
    /// let mut slice = vec.vecslice(1..=3); 
    ///
    /// slice.sort_unstable();
    /// assert_eq!(slice, [-3, 2, 4]);
    /// assert_eq!(vec, [-5, -3, 2, 4, 1]);
    /// ```
    pub fn sort_unstable(&mut self) where T: Ord {
        self.vec[self.start..self.end].sort_unstable();
    }
}

impl<T> Extend<T> for VecSlice<'_, T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.vec.splice(self.end..self.end, iter);
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for VecSlice<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VecSlice").field("slice", &&self.vec[self.start..self.end]).field("start", &self.start).field("end", &self.end).field("vec", &self.vec).finish()
    }
}

pub trait Slice<T> {
    fn vecslice(&mut self, range: impl RangeBounds<usize>) -> VecSlice<T>;
    
    fn vecslice_at_tail(&mut self) -> VecSlice<T>;
}

impl<T> Slice<T> for Vec<T> {
    /// Creates a new [`VecSlice`] on the 
    /// 
    /// The new slice will be empty, and newly added elements will be appended to the end of the [`VecSlice`].
    fn vecslice(&mut self, range: impl RangeBounds<usize>) -> VecSlice<T> {
        VecSlice::new(range, self)
    }
    
    /// Creates a new [`VecSlice`] at the tail of the [`Vec`].
    /// 
    /// The new slice will be empty, and newly added elements will be appended to the end of the [`Vec`].
    fn vecslice_at_tail(&mut self) -> VecSlice<T> {
        self.vecslice(self.len()..)
    }
}

impl<T: PartialEq> PartialEq for VecSlice<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.vec[self.start..self.end] == other.vec[other.start..other.end]
    }
}

impl<T: PartialOrd> PartialOrd for VecSlice<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.vec[self.start..self.end].partial_cmp(&other.vec[other.start..other.end])
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for VecSlice<'_, T> {
    fn eq(&self, other: &[T; N]) -> bool {
        &self.vec[self.start..self.end] == other
    }
}

impl<T: PartialOrd, const N: usize> PartialOrd<[T; N]> for VecSlice<'_, T> {
    fn partial_cmp(&self, other: &[T; N]) -> Option<core::cmp::Ordering> {
        self.vec[self.start..self.end].partial_cmp(other)
    }
}

impl<T: PartialEq> PartialEq<&[T]> for VecSlice<'_, T> {
    fn eq(&self, other: &&[T]) -> bool {
        &self.vec[self.start..self.end] == *other
    }
}

impl<T: PartialOrd> PartialOrd<&[T]> for VecSlice<'_, T> {
    fn partial_cmp(&self, other: &&[T]) -> Option<core::cmp::Ordering> {
        self.vec[self.start..self.end].partial_cmp(other)
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for VecSlice<'_, T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        &self.vec[self.start..self.end] == other
    }
}

impl<T: PartialOrd> PartialOrd<Vec<T>> for VecSlice<'_, T> {
    fn partial_cmp(&self, other: &Vec<T>) -> Option<core::cmp::Ordering> {
        self.vec[self.start..self.end].partial_cmp(other)
    }
}

impl<T> core::borrow::Borrow<[T]> for VecSlice<'_, T> {
    fn borrow(&self) -> &[T] {
        &self.vec[self.start..self.end]
    }
}

impl<T> core::borrow::BorrowMut<[T]> for VecSlice<'_, T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self.vec[self.start..self.end]
    }
}