pub struct FixedSizeArrayPool {
    cached_buffers: Vec<Vec<usize>>,
    segment_size: usize,
    max_array_count: usize,
}
impl FixedSizeArrayPool {
    pub fn len(&self) -> usize {
        self.cached_buffers.len()
    }
    pub fn new(segment_size: usize, max_array_count: usize) -> FixedSizeArrayPool {
        FixedSizeArrayPool {
            cached_buffers: vec![vec![0; segment_size]; max_array_count],
            segment_size: segment_size,
            max_array_count: max_array_count,
        }
    }

    pub fn acquire(&mut self) -> Vec<usize> {
        if self.cached_buffers.len() > 0 {
            self.cached_buffers.pop().unwrap()
        } else {
            vec![0; self.segment_size]
        }
    }

    pub fn release(&mut self, buffer: Vec<usize>) -> bool {
        // we will add back the vector and validate if it matches the size
        if buffer.len() == self.segment_size && self.cached_buffers.len() < self.max_array_count {
            self.cached_buffers.push(buffer);
            true
        } else {
            false
        }
    }
}
