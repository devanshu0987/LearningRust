/// Represents a simple bit vector which encodes bit
/// information inside u32 ints and does bit manipulation
/// to maintain a set of bits
pub struct SimpleBitVector {
    /// Stores u32 each representing 32 bits where we can set bits.
    slots: Vec<u32>,
    pub length: u32,
    pub allocated_length: u32,
}

pub struct SimpleBitVectorConsumableIterator {
    vector: SimpleBitVector,
    index: u32,
}

impl SimpleBitVector {
    /// Creates a new instance of SimpleBitVector
    /// length - 1 ints will be indexed
    pub fn new(length: u32, default_value: bool) -> Self {
        assert!(length > 0, "Wrong argument: Length cannot be zero");
        let slots_required: u32 = ((length + 31) / 32).try_into().unwrap();
        // if default_value is set to true, it means all bits are set
        // hence each slot should hold the max which is 11111...
        // if false, then we set to 0
        let internal_default_value = match default_value {
            true => u32::MAX,
            false => 0,
        };

        SimpleBitVector {
            slots: vec![internal_default_value; slots_required.try_into().unwrap()],
            length: length,
            allocated_length: slots_required,
        }
    }

    /// Tries to get value, if out of index, returns None
    pub fn try_get(&self, index: u32) -> Option<bool> {
        // index should be one less than len * 32
        if index >= self.length {
            None
        } else {
            Some(self.get_internal(index))
        }
    }

    /// Tries to get value, if out of index, panics.
    pub fn get(&self, index: u32) -> bool {
        // index should be one less than len * 32
        assert!(index < self.length, "Invalid index");

        self.get_internal(index)
    }

    /// infallible get. No index check if performed.
    fn get_internal(&self, index: u32) -> bool {
        let slot_index: usize = (index / 32).try_into().unwrap();
        let inter_slot_index = index % 32;
        let slot_u32 = self.slots[slot_index];

        // bit magic that I don't understand fully.
        slot_u32 & (1 << inter_slot_index) != 0
    }

    pub fn set(&mut self, index: u32, value: bool) {
        assert!(index < self.length, "Invalid index");

        self.set_internal(index, value);
    }

    pub fn try_set(&mut self, index: u32, value: bool) -> bool {
        // index should be one less than len * 32
        if index >= self.length {
            false
        } else {
            self.set_internal(index, value);
            true
        }
    }

    /// infallible function
    fn set_internal(&mut self, index: u32, value: bool) {
        let slot_index: usize = (index / 32).try_into().unwrap();
        let inter_slot_index = index % 32;
        let mut slot_u32 = self.slots[slot_index];

        if value {
            slot_u32 |= 1 << inter_slot_index;
        } else {
            slot_u32 &= !(1 << inter_slot_index);
        }
        self.slots[slot_index] = slot_u32;
    }
}

impl IntoIterator for SimpleBitVector {
    type Item = bool;
    type IntoIter = SimpleBitVectorConsumableIterator;

    fn into_iter(self) -> Self::IntoIter {
        SimpleBitVectorConsumableIterator {
            vector: self,
            index: 0,
        }
    }
}

impl Iterator for SimpleBitVectorConsumableIterator {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.index < self.vector.length {
            let result = self.vector.get_internal(self.index);
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}
