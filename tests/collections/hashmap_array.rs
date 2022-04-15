use common::collections::hashmap_array::HashmapArray;

#[test]
fn happy_path() {
    let mut array_of_hashmaps = HashmapArray::<u8, u8>::new(16);
    let _ = array_of_hashmaps.insert(1, 2);
    let value = array_of_hashmaps.try_get(&1);
    assert!(value.is_some(), "Try get failed");
    let unwrapped_value = value.unwrap();
    assert!(
        *unwrapped_value == 2,
        "Value retrieved is different from value inserted"
    );
}
