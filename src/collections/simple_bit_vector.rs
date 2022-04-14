/// Represents a simple bit vector which encodes bit
/// information inside u32 ints and does bit manipulation
/// to maintain a set of bits
pub struct SimpleBitVector {
    /// Stores u32 each representing 32 bits where we can set bits.
    slots: Vec<u32>,
    pub length: usize,
}

impl SimpleBitVector {
    /// Creates a new instance of SimpleBitVector
    /// length - 1 ints will be indexed
    pub fn new(length: u32, default_value: bool) -> Self {
        assert!(length > 0, "Wrong argument: Length cannot be zero");
        let slots_required: usize = ((length + 31) / 32).try_into().unwrap();
        // if default_value is set to true, it means all bits are set
        // hence each slot should hold the max which is 11111...
        // if false, then we set to 0
        let internal_default_value = match default_value {
            true => u32::MAX,
            false => 0,
        };

        SimpleBitVector {
            slots: vec![internal_default_value; slots_required],
            length: slots_required,
        }
    }

    pub fn get(&self, index: u32) -> bool {
        // index should be one less than len * 32
        assert!(index < (self.slots.len() * 32).try_into().unwrap(), "Invalid index");

        let slot_index: usize = (index / 32).try_into().unwrap();
        let inter_slot_index = index % 32;
        let slot_u32 = self.slots[slot_index];

        // bit magic
        slot_u32 & (1 << inter_slot_index) != 0
    }

    pub fn set(&mut self, index: u32, value: bool) {
        assert!(index < (self.slots.len() * 32).try_into().unwrap(), "Invalid index");

        let slot_index: usize = (index / 32).try_into().unwrap();
        let inter_slot_index = index % 32;
        let mut slot_u32 = self.slots[slot_index];

        if value {
            slot_u32 |= 1 << inter_slot_index;
        }
        else {
            slot_u32 &= !(1 << inter_slot_index);
        }
        self.slots[slot_index] = slot_u32;
    }
}
