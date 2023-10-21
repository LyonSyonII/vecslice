//! This module provides the [`VecSlice`] struct, which represents a growable mutable reference on a [`Vec`].
//!
//! The [`VecSlice`] struct allows you to work with a mutable slice of a [`Vec`] while ensuring memory safety by requiring a mutable reference to the underlying buffer, only one [`VecSlice`] can exist at a time, preventing multiple mutable references and potential data races.
//!
//! # Complexity
//!
//! All operations on a [`VecSlice`] have O(n) complexity, as the slice can start and end anywhere on the original vector.
//!
//! If you use [`VecSlice::new_at_tail`] to create a slice, the complexity of push operations on the new slice will be O(1).
//!
//! # Examples
//!
//! ```
//! use vecslice::Slice;
//!
//! let mut vec = vec![1, 2, 3];
//!
//! // Create a VecSlice that slices the first two elements
//! let mut slice = vec.vecslice(..2);
//!
//! // Perform operations on the slice
//! assert_eq!(slice.len(), 2);
//! assert_eq!(slice, [1, 2]);
//!
//! slice.push_back(3);
//! slice.push_front(0);
//!
//! assert_eq!(slice, [0, 1, 2, 3]);
//!
//! // The original `Vec` is also modified
//! assert_eq!(vec, [0, 1, 2, 3, 3]);
//!
//! // Create a new VecSlice that slices [1, 2, 3]
//! let mut slice = vec.vecslice(1..=3);
//!
//! // Perform operations on the slice
//! assert_eq!(slice.pop_back(), Some(3));
//! assert_eq!(slice.pop_front(), Some(1));
//! assert_eq!(slice, [2]);
//!
//! // The original `Vec` is also modified
//! assert_eq!(vec, [0, 2, 3]);
//!
//! ```
//! For more information, see the [`VecSlice`] struct documentation.

use core::ops::RangeBounds;

mod drain;
mod index;
mod iter;

/// Growable mutable reference on a [`Vec`].
///
/// Due to requiring a mutable reference to the underlying buffer, only one [`VecSlice`] can exist at a time, ensuring memory safety.
///
/// # Complexity
///
/// All operations have O(n) complexity, as the slice can start and end anywhere on the original vector.
///
/// If [`VecSlice::new_at_tail`] is used, the complexity of push_back operations on the new slice will be O(1).
///
/// # Examples
///
/// ```
/// use vecslice::Slice;
///
/// let mut vec = vec![1, 2, 3];
/// let mut slice = vec.vecslice(0..=1);
/// assert_eq!(slice.len(), 2);
/// assert_eq!(slice, [1, 2]);
///
/// slice.push_back(3);
/// assert_eq!(slice, [1, 2, 3]);
/// assert_eq!(vec, [1, 2, 3, 3]);
/// ```
// #[derive(Eq, Ord)]
pub struct VecSlice<'a, T> {
    start: usize,
    end: usize,
    original: &'a mut dyn Sliceable<T>,
}

pub trait Slice<T>: Sliceable<T>
where
    Self: Sized,
{
    fn vecslice(&mut self, range: impl core::ops::RangeBounds<usize>) -> VecSlice<'_, T> {
        VecSlice::new(range, self as &mut dyn Sliceable<T>)
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
    /// let mut slice = vec.vecslice(..);
    /// assert_eq!(slice, [1, 2, 3]);
    ///
    /// let mut slice2 = slice.vecslice_at_tail();
    /// assert_eq!(slice2, []);
    /// slice2.push_back(4);
    /// assert_eq!(slice2, [4]);
    ///
    /// assert_eq!(slice, [1, 2, 3, 4]);
    /// assert_eq!(vec, [1, 2, 3, 4]);
    /// ```
    fn vecslice_at_tail(&mut self) -> VecSlice<'_, T> {
        self.vecslice(self.len()..self.len())
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
    /// let mut slice2 = slice.vecslice_at_head();
    /// assert_eq!(slice2, []);
    /// slice2.push_back(4);
    /// assert_eq!(slice2, [4]);
    ///
    /// // assert_eq!(slice, [1, 2, 3]);
    /// assert_eq!(vec, [4, 1, 2, 3]);
    /// ```
    fn vecslice_at_head(&mut self) -> VecSlice<'_, T> {
        self.vecslice(0..0)
    }
}

impl<T, S> Slice<T> for S where S: Sliceable<T> {}

#[allow(clippy::len_without_is_empty)]
pub trait Sliceable<T>: AsRef<[T]> + AsMut<[T]> {
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
    fn insert(&mut self, index: usize, value: T);
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
    fn remove(&mut self, index: usize) -> T;
    /// Returns the length of the slice.
    fn len(&self) -> usize;
}

impl<'a, T> VecSlice<'a, T> {
    fn translate_range(range: impl RangeBounds<usize>, start: usize, end: usize) -> (usize, usize) {
        use core::ops::Bound::*;
        match (range.start_bound(), range.end_bound()) {
            (Included(&s), Included(&e)) => (start + s, start + e + 1),
            (Included(&s), Excluded(&e)) => (start + s, start + e),
            (Included(&s), Unbounded) => (start + s, end),
            (Excluded(&s), Included(&e)) => (start + s + 1, start + e + 1),
            (Excluded(&s), Excluded(&e)) => (start + s + 1, start + e),
            (Excluded(&s), Unbounded) => (start + s + 1, end),
            (Unbounded, Included(&e)) => (start, start + e + 1),
            (Unbounded, Excluded(&e)) => (start, start + e),
            (Unbounded, Unbounded) => (start, end),
        }
    }

    pub fn new(
        range: impl core::ops::RangeBounds<usize>,
        original: &'a mut dyn Sliceable<T>,
    ) -> VecSlice<'a, T> {
        let (start, end) = VecSlice::<T>::translate_range(range, 0, original.len());
        VecSlice {
            start,
            end,
            original,
        }
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
    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len());
        self.original.insert(self.start + index, value);
        self.end += 1;
    }
    /// Appends an element to the back of a collection.
    ///
    /// If you'd like to push at the front of the collection, use [`VecSlice::push_front`] instead.
    ///
    /// See [`VecSlice`] for the time complexity.
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
    /// slice.push_back(4);
    /// slice.push_back(5);
    /// assert_eq!(slice, [1, 2, 4, 5]);
    /// assert_eq!(vec, [0, 1, 2, 4, 5, 3]);
    /// ```
    pub fn push_back(&mut self, element: T) {
        self.insert(self.len(), element);
    }
    /// Appends an element to the front of the collection.
    ///
    /// See [`VecSlice`] for the time complexity.
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
        self.insert(0, element);
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
        self.original.remove(self.start + index)
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
    /// use vecslice::Slice;
    ///
    /// let mut vec = vec![0, 1, 2, 3];
    /// let mut slice = vec.vecslice(..=2);
    /// assert_eq!(slice, [0, 1, 2]);
    /// assert_eq!(slice.pop_back(), Some(2));
    /// assert_eq!(slice, [0, 1]);
    /// assert_eq!(vec, [0, 1, 3]);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        if !self.is_empty() {
            Some(self.remove(self.len() - 1))
        } else {
            None
        }
    }
    /// Removes the first element from a VecSlice and returns it, or [`None`] if it
    /// is empty.
    ///
    /// If you'd like to pop the last element, use [`VecSlice::pop_back`] instead.
    ///
    /// [`VecSlice::pop_back`]: crate::VecSlice::pop_back
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    ///
    /// let mut vec = vec![0, 1, 2, 3];
    /// let mut slice = vec.vecslice(1..);
    /// assert_eq!(slice.pop_front(), Some(1));
    /// assert_eq!(slice, [2, 3]);
    /// assert_eq!(vec, [0, 2, 3]);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if !self.is_empty() {
            Some(self.remove(0))
        } else {
            None
        }
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
    pub fn drain<'borrow>(&'borrow mut self, range: impl RangeBounds<usize>) -> crate::drain::Drain<'a, 'borrow, T> {
        crate::drain::Drain::new(self, range)
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
    /// Returns `true` if the slice is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.as_ref().to_vec()
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
    pub fn into_vec(self) -> Vec<T>
    where
        T: Clone,
    {
        self.as_ref().to_vec()
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
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.as_mut().sort()
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
    pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.as_mut().sort_unstable()
    }
    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5];
    /// let slice = vec.vecslice(1..=3);
    /// assert_eq!(slice, [1, 2, 3]);
    ///
    /// let mut iterator = slice.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&3));
    /// assert_eq!(iterator.next(), None);
    ///
    /// // equivalent syntax
    /// let mut copy = Vec::new();
    /// for elem in &slice {
    ///     copy.push(elem);
    /// }
    ///
    /// assert_eq!(copy, [&1, &2, &3]);
    /// ```
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_ref().iter()
    }
    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5];
    /// let mut slice = vec.vecslice(1..=3);
    /// assert_eq!(slice, [1, 2, 3]);
    ///
    /// for elem in slice.iter_mut() {
    ///     *elem += 2;
    /// }
    /// assert_eq!(slice, [3, 4, 5]);
    ///
    /// // equivalent syntax
    /// for elem in &mut slice {
    ///    *elem *= -1;
    /// }
    /// assert_eq!(slice, [-3, -4, -5]);
    /// assert_eq!(vec, [0, -3, -4, -5, 4, 5]);
    /// ```
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.as_mut().iter_mut()
    }
    /// Consumes `self` and returns a mutable reference to the original slice.
    pub fn into_original(self) -> &'a mut dyn Sliceable<T> {
        self.original
    }
    /// Returns the length of the slice.
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'a, T> Sliceable<T> for VecSlice<'a, T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn insert(&mut self, index: usize, value: T) {
        self.insert(index, value)
    }

    fn remove(&mut self, index: usize) -> T {
        self.remove(index)
    }
}

impl<T> Extend<T> for VecSlice<'_, T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        // TODO: Use splice instead
        for elem in iter {
            self.push_back(elem)
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for VecSlice<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.as_ref();
        f.debug_struct("VecSlice")
            .field("slice", &v)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("original", &self.original.as_ref())
            .finish()
    }
}

impl<T, Rhs> PartialEq<Rhs> for VecSlice<'_, T>
where
    T: PartialEq,
    Rhs: AsRef<[T]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T, Rhs> PartialOrd<Rhs> for VecSlice<'_, T>
where
    T: PartialOrd,
    Rhs: AsRef<[T]>,
{
    fn partial_cmp(&self, other: &Rhs) -> Option<core::cmp::Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<T> core::borrow::Borrow<[T]> for VecSlice<'_, T> {
    fn borrow(&self) -> &[T] {
        self.as_ref()
    }
}

impl<T> core::borrow::BorrowMut<[T]> for VecSlice<'_, T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut()
    }
}

impl<T> AsRef<[T]> for VecSlice<'_, T> {
    fn as_ref(&self) -> &[T] {
        &self.original.as_ref()[self.start..self.end]
    }
}

impl<T> AsMut<[T]> for VecSlice<'_, T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.original.as_mut()[self.start..self.end]
    }
}

impl<'a, T> From<&'a mut dyn Sliceable<T>> for VecSlice<'a, T> {
    fn from(original: &'a mut dyn Sliceable<T>) -> Self {
        Self::new(.., original)
    }
}

impl<T> Sliceable<T> for Vec<T> {
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
