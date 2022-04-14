pub struct FixedSizeArrayPool<T> {
    cached_buffers: Vec<Vec<T>>,
    segment_size: usize,
    max_array_count: usize,
}
impl<T> FixedSizeArrayPool<T> {
    pub fn len(&self) -> usize {
        self.cached_buffers.len()
    }
    pub fn new(segment_size: usize, max_array_count: usize) -> FixedSizeArrayPool<T>
    where
        T: Clone,
        T: Default,
    {
        FixedSizeArrayPool {
            cached_buffers: vec![vec![T::default(); segment_size]; max_array_count],
            segment_size: segment_size,
            max_array_count: max_array_count,
        }
    }

    pub fn acquire(&mut self) -> Vec<T>
    where
        T: Clone,
        T: Default,
    {
        if self.cached_buffers.len() > 0 {
            self.cached_buffers.pop().unwrap()
        } else {
            vec![T::default(); self.segment_size]
        }
    }

    pub fn release(&mut self, buffer: Vec<T>) -> bool {
        // we will add back the vector and validate if it matches the size
        if buffer.len() == self.segment_size && self.cached_buffers.len() < self.max_array_count {
            self.cached_buffers.push(buffer);
            true
        } else {
            false
        }
    }
}
