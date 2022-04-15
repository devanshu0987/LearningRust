/// Maintains pool of fixed size arrays
/// FixedSizeArrayPool<Type, SegmentSize: usize, PoolSize: usize>
pub struct FixedSizeArrayPool<T, const SS: usize, const PS: usize> {
    cached_buffers: Vec<[T; SS]>,
}

pub enum Response {
    Ok,
    PoolSaturated,
}

impl<T, const SS: usize, const PS: usize> FixedSizeArrayPool<T, SS, PS> {
    pub fn len(&self) -> usize {
        self.cached_buffers.len()
    }

    pub fn new() -> FixedSizeArrayPool<T, SS, PS>
    where
        T: Clone,
        T: Copy,
        T: Default,
    {
        let temp = [T::default(); SS];
        FixedSizeArrayPool {
            cached_buffers: vec![temp.clone(); PS],
        }
    }

    pub fn acquire(&mut self) -> [T; SS]
    where
        T: Clone,
        T: Copy,
        T: Default,
    {
        return self
            .cached_buffers
            .pop()
            .unwrap_or_else(|| [T::default(); SS]);
    }

    pub fn release(&mut self, buffer: [T; SS]) -> Response {
        // we will add back the vector and validate if it matches the size
        if self.cached_buffers.len() < PS {
            self.cached_buffers.push(buffer);
            Response::Ok
        } else {
            Response::PoolSaturated
        }
    }
}
