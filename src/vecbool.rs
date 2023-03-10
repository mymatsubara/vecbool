//! Module with the [VecBool] implementation

/// Underlying datatype to store the bits
type Chunk = u8;
const CHUNK_SIZE: usize = 8;

/// Wrapper around [Vec<u8>]. You can use it similarly to a `Vec<bool>`.
pub struct VecBool {
    len: usize,
    chunks: Vec<Chunk>,
}

impl VecBool {
    #[inline]
    /// Creates a new empty [VecBool].
    ///
    /// Does not allocate memory on heap until elements are added.
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            len: 0,
        }
    }

    #[inline]
    // Create a [VecBool] with preallocated memory.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            chunks: Vec::with_capacity((capacity / CHUNK_SIZE) + 1),
            len: 0,
        }
    }

    #[inline]
    // Create a [VecBool] with all bits set to `0`
    pub fn with_zeros(len: usize) -> Self {
        Self {
            chunks: vec![0; (len / CHUNK_SIZE) + 1],
            len,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.chunks.len() * CHUNK_SIZE
    }

    #[inline]
    /// Get the boolean value stored on `index`. If outbounds, return [None]
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len {
            return None;
        }

        Some(self.get_unchecked(index))
    }

    #[inline]
    /// Get the boolean value from `index` position. This method **panics** if `index` is out of bounds.
    pub fn get_unchecked(&self, index: usize) -> bool {
        let (chunk_index, mask) = VecBool::get_index(index);

        let bits = self.chunks[chunk_index];

        (bits & mask) != 0
    }

    #[inline]
    /// Set the boolean value in `index`. Returns `false` if `index` is out of bounds.
    pub fn set(&mut self, index: usize, value: bool) -> bool {
        if index >= self.len {
            return false;
        }

        self.set_unchecked(index, value);

        true
    }

    #[inline]
    /// Set the boolean value in `index`. This method **panics** if `index` is out of bounds.
    pub fn set_unchecked(&mut self, index: usize, value: bool) {
        let (chunk_index, mask) = VecBool::get_index(index);

        if value {
            self.chunks[chunk_index] |= mask;
        } else {
            self.chunks[chunk_index] &= !mask;
        }
    }

    #[inline]
    /// Push an `bool` to the end of vector.
    pub fn push(&mut self, value: bool) {
        if self.len >= self.capacity() {
            self.chunks.push(0)
        }

        self.len += 1;
        self.set_unchecked(self.len - 1, value);
    }

    #[inline]
    /// Remove the last `bool` value from the vector. If the vector is empty, it return [None] otherwise it returns
    /// the removed value.
    pub fn pop(&mut self) -> Option<bool> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        let data = self.get_unchecked(self.len);

        if self.len % CHUNK_SIZE == 0 {
            self.chunks.pop();
        }

        Some(data)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = bool> + '_ {
        self.chunks
            .iter()
            .take(self.len / CHUNK_SIZE)
            .flat_map(|chunk| (0..CHUNK_SIZE).map(move |shift| chunk & (1 << shift) != 0))
            .chain({
                let chunk = self.chunks.last().copied().unwrap_or_default();
                (0..(self.len % CHUNK_SIZE)).map(move |shift| chunk & (1 << shift) != 0)
            })
    }

    #[inline]
    fn get_index(index: usize) -> (usize, Chunk) {
        let chunk_index = index / CHUNK_SIZE;
        let shifts = index % CHUNK_SIZE;
        let mask = 1 << shifts;

        (chunk_index, mask)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let mask = VecBool::new();
        simple_test(mask);
    }

    #[test]
    fn with_capacity() {
        let mask = VecBool::with_capacity(CHUNK_SIZE * 4);
        simple_test(mask);
    }

    fn simple_test(mut mask: VecBool) {
        assert_eq!(mask.get(0), None);

        mask.push(true);
        assert_eq!(mask.get(0), Some(true));
        assert_eq!(mask.get(1), None);

        mask.push(false);
        assert_eq!(mask.get(0), Some(true));
        assert_eq!(mask.get(1), Some(false));
        assert_eq!(mask.get(2), None);

        mask.set(0, false);
        mask.set(1, true);
        assert_eq!(mask.iter().collect::<Vec<_>>(), vec![false, true]);

        assert_eq!(mask.pop(), Some(true));
        assert_eq!(mask.pop(), Some(false));
        assert_eq!(mask.pop(), None);
        assert_eq!(mask.iter().collect::<Vec<_>>(), vec![]);

        let size = CHUNK_SIZE * 4;
        for i in 0..size {
            mask.push(i % 3 == 0);
        }

        assert_eq!(
            mask.iter().collect::<Vec<_>>(),
            (0..size).map(|i| i % 3 == 0).collect::<Vec<_>>()
        )
    }

    #[test]
    fn with_len() {
        let len = 16;
        let mask = VecBool::with_zeros(len);

        for i in 0..len {
            assert_eq!(mask.get(i), Some(false));
        }

        assert_eq!(mask.get(len), None);
    }
}
