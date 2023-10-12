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
pub struct VecSlice<'a, T, S>
where
    S: Slice<T> + ?Sized,
{
    start: usize,
    end: usize,
    original: &'a mut S,
    _type: core::marker::PhantomData<T>,
}

impl<'a, T, S> VecSlice<'a, T, S>
where
    S: Slice<T> + ?Sized,
{
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
        original: &'a mut S,
    ) -> VecSlice<'a, T, S> {
        let (start, end) = VecSlice::<T, S>::translate_range(range, 0, original.len());
        VecSlice {
            start,
            end,
            original,
            _type: core::marker::PhantomData,
        }
    }
}

impl<'a, T, S> Slice<T> for VecSlice<'a, T, S>
where
    S: Slice<T> + ?Sized,
{
    type Drain<'b> = S::Drain<'b> where Self: 'b;

    fn vecslice(&mut self, range: impl core::ops::RangeBounds<usize>) -> VecSlice<'_, T, Self> {
        VecSlice::new(range, self)
    }

    fn len(&self) -> usize {
        self.end - self.start
    }

    fn insert(&mut self, index: usize, element: T) {
        assert!(index <= self.len());
        self.original.insert(self.start + index, element);
        self.end += 1;
    }

    fn remove(&mut self, index: usize) -> T {
        assert!(index <= self.len());
        self.end -= 1;
        self.original.remove(self.start + index)
    }

    fn drain(&mut self, range: impl RangeBounds<usize>) -> Self::Drain<'_> {
        let (start, end) = Self::translate_range(range, self.start, self.end);
        self.end -= end - start; // Adjust length of the new slice
        self.original.drain(start..end)
    }

    fn as_slice(&self) -> &[T] {
        &self.original.as_slice()[self.start..self.end]
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.original.as_mut_slice()[self.start..self.end]
    }
}

impl<T, S> Extend<T> for VecSlice<'_, T, S>
where
    S: Slice<T> + Extend<T>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        // TODO: Wrong, should use splice
        self.original.extend(iter)
    }
}

impl<T: std::fmt::Debug, S: std::fmt::Debug + Slice<T>> std::fmt::Debug for VecSlice<'_, T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.as_slice();
        f.debug_struct("VecSlice")
            .field("slice", &v)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("original", &self.original)
            .finish()
    }
}

pub trait Slice<T> {
    type Drain<'b>
    where
        Self: 'b;

    fn vecslice(&mut self, range: impl core::ops::RangeBounds<usize>) -> VecSlice<'_, T, Self>;

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
    fn vecslice_at_tail(&mut self) -> VecSlice<'_, T, Self> {
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
    /// let mut slice2 = slice.vecslice_at_front();
    /// assert_eq!(slice2, []);
    /// slice2.push_back(4);
    /// assert_eq!(slice2, [4]);
    ///
    /// // assert_eq!(slice, [1, 2, 3]);
    /// assert_eq!(vec, [4, 1, 2, 3]);
    /// ```
    fn vecslice_at_front(&mut self) -> VecSlice<'_, T, Self> {
        self.vecslice(0..0)
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
    fn insert(&mut self, index: usize, element: T);
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
    /// slice.push_back(4);
    /// slice.push_back(5);
    /// assert_eq!(slice, [1, 2, 4, 5]);
    /// assert_eq!(vec, [0, 1, 2, 4, 5, 3]);
    /// ```
    fn push_back(&mut self, element: T) {
        self.insert(self.len(), element);
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
    fn push_front(&mut self, element: T) {
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
    fn remove(&mut self, index: usize) -> T;
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
    fn pop_back(&mut self) -> Option<T> {
        if self.len() > 0 {
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
    fn pop_front(&mut self) -> Option<T> {
        if self.len() > 0 {
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
    fn drain(&mut self, range: impl RangeBounds<usize>) -> Self::Drain<'_>;
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
    fn clear(&mut self) {
        self.drain(..);
    }

    /// Returns the length of the slice.
    fn len(&self) -> usize;
    /// Returns `true` if the slice is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns a reference to the underlying slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use vecslice::Slice;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// 
    /// let slice = vec.vecslice(0..2);
    /// assert_eq!(slice, [1, 2]);
    /// assert_eq!(slice.as_slice(), [1, 2]);
    /// 
    /// let slice = vec.vecslice(1..=2);
    /// assert_eq!(slice, [2, 3]);
    /// assert_eq!(slice.as_slice(), [2, 3]);
    /// ```
    fn as_slice(&self) -> &[T];
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
    fn as_mut_slice(&mut self) -> &mut [T];
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
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.as_slice().to_vec()
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
    fn into_vec(self) -> Vec<T>
    where
        Self: Sized + IntoIterator<Item = T>,
    {
        self.into_iter().collect()
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
    fn sort(&mut self)
    where
        T: Ord,
    {
        self.as_mut_slice().sort()
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
    fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.as_mut_slice().sort_unstable()
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
    fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
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
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }
}

impl<T> Slice<T> for Vec<T> {
    type Drain<'b> = std::vec::Drain<'b, T> where T: 'b;

    fn vecslice(&mut self, range: impl core::ops::RangeBounds<usize>) -> VecSlice<'_, T, Self> {
        VecSlice::new(range, self)
    }

    fn insert(&mut self, index: usize, element: T) {
        self.insert(index, element)
    }

    fn remove(&mut self, index: usize) -> T {
        self.remove(index)
    }

    fn drain(&mut self, range: impl RangeBounds<usize>) -> Self::Drain<'_> {
        self.drain(range)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, S> PartialEq for VecSlice<'_, T, S>
where
    T: PartialEq,
    S: Slice<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T, S> PartialOrd for VecSlice<'_, T, S>
where
    T: PartialOrd,
    S: Slice<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T, S, const N: usize> PartialEq<[T; N]> for VecSlice<'_, T, S>
where
    T: PartialEq,
    S: Slice<T>,
{
    fn eq(&self, other: &[T; N]) -> bool {
        self.as_slice() == other
    }
}

impl<T, S, const N: usize> PartialOrd<[T; N]> for VecSlice<'_, T, S>
where
    T: PartialOrd,
    S: Slice<T>,
{
    fn partial_cmp(&self, other: &[T; N]) -> Option<core::cmp::Ordering> {
        self.as_slice().partial_cmp(other)
    }
}

impl<T, S> PartialEq<&[T]> for VecSlice<'_, T, S>
where
    T: PartialEq,
    S: Slice<T>,
{
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice() == *other
    }
}

impl<T, S> PartialOrd<&[T]> for VecSlice<'_, T, S>
where
    T: PartialOrd,
    S: Slice<T>,
{
    fn partial_cmp(&self, other: &&[T]) -> Option<core::cmp::Ordering> {
        self.as_slice().partial_cmp(other)
    }
}

impl<T, S: Slice<T>> core::borrow::Borrow<[T]> for VecSlice<'_, T, S> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, S: Slice<T>> core::borrow::BorrowMut<[T]> for VecSlice<'_, T, S> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, S: Slice<T>> AsRef<[T]> for VecSlice<'_, T, S> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, S: Slice<T>> AsMut<[T]> for VecSlice<'_, T, S> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
