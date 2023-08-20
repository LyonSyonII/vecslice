use crate::VecSlice;

impl<'a, T> VecSlice<'a, T> {
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
        self.into_iter()
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
        self.into_iter()
    }
}

impl<'a, T> IntoIterator for VecSlice<'a, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.vec[self.start..self.end].iter()
    }
}

impl<'a, T> IntoIterator for &'a VecSlice<'_, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.vec[self.start..self.end].iter()
    }
}

impl<'a, T> IntoIterator for &'a mut VecSlice<'_, T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.vec[self.start..self.end].iter_mut()
    }
}

impl<'a, T> From<&'a mut Vec<T>> for VecSlice<'a, T> {
    fn from(vec: &'a mut Vec<T>) -> Self {
        Self::new(.., vec)
    }
}