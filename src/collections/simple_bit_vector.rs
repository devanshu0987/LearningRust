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
    /// length denotes
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
}
