use std::{
    cmp::{max, min},
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

pub struct HashmapArray<TKey, TValue>
where
    TKey: Hash + Eq,
{
    partitions: Vec<HashMap<TKey, TValue>>,
}

impl<TKey, TValue> HashmapArray<TKey, TValue>
where
    TKey: Hash + Eq,
{
    /// Minimum 16 partitions
    /// Maximum 256 partitions
    pub fn new(mut partition_count: usize) -> Self {
        partition_count = max(partition_count, 16);
        partition_count = min(partition_count, 256);
        let mut container = Vec::with_capacity(partition_count);
        for _ in [0..16] {
            container.push(HashMap::<TKey, TValue>::new());
        }
        HashmapArray {
            partitions: container,
        }
    }

    /// TODO: Implement other Hashmap related APIs which are relevant
    /// Entry API is new and cool
    /// I wonder how can I implement that.

    /// Uses Hashmap insert
    /// If the map did not have this key present, None is returned.
    /// If the map did have this key present, the value is updated, and the old value is returned
    pub fn insert(&mut self, key: TKey, value: TValue) {
        let slot = self.get_partition(&key);
        self.partitions[slot].insert(key, value);
    }

    pub fn try_get(&self, key: &TKey) -> Option<&TValue> {
        let slot = self.get_partition(key);
        self.partitions[slot].get(key)
    }

    /// TODO: Allow different strategies for finding slot for a particular key
    /// Round Robin can also be a good option
    /// TODO: Can we pass closure here?
    fn get_partition(&self, key: &TKey) -> usize {
        // do some magic here
        let total_partitions = self.partitions.len();
        // find hash of key and then assign a slot for it
        // TODO: I am not sure about the cost of new here.
        // I wish we could cache this hasher and reuse it
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        // truncate hash to usize
        let hash: usize = s.finish().try_into().unwrap();
        // Assumption here is that the distribution will not be skewed
        hash % total_partitions
    }
}
