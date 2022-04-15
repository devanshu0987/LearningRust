use common::collections::simple_bit_vector::SimpleBitVector;
#[test]
fn new_works() {
    assert!(SimpleBitVector::new(32, false).allocated_length == 1);
    assert!(SimpleBitVector::new(31, true).allocated_length == 1);
    assert!(SimpleBitVector::new(1, true).allocated_length == 1);
}

#[test]
#[should_panic]
fn new_fails() {
    let _ = SimpleBitVector::new(0, false);
}

#[test]
fn new_fails_v2() {
    let result = std::panic::catch_unwind(|| {
        let _ = SimpleBitVector::new(0, false);
    });
    assert!(result.is_err());
}

#[test]
fn set_get_single_success() {
    let mut bv = SimpleBitVector::new(32, false);
    assert!(bv.get(0) == false, "Index 0 is already set");
    bv.set(0, true);
    assert!(bv.get(0) == true, "Index 0 is not set");
    bv.set(0, false);
    assert!(bv.get(0) == false, "Index 0 is set");
}

#[test]
fn set_get_multiple_success() {
    let mut bv = SimpleBitVector::new(32, false);
    for index in &[1u32..5] {
        assert!(
            bv.get((*index).start) == false,
            "Index {} is already set",
            (*index).start
        );
        bv.set((*index).start, true);
    }

    for index in &[1u32..5] {
        assert!(
            bv.get((*index).start) == true,
            "Index {} is not set",
            (*index).start
        );
    }
}

#[test]
fn set_index_out_of_bounds_panics() {
    let result = std::panic::catch_unwind(|| {
        let mut bv = SimpleBitVector::new(32, false);
        bv.set(32, true);
    });
    assert!(result.is_err());
}

#[test]
fn get_index_out_of_bounds_panics() {
    let result = std::panic::catch_unwind(|| {
        let bv = SimpleBitVector::new(32, false);
        bv.get(32);
    });
    assert!(result.is_err());
}

#[test]
fn try_set_index_out_of_bounds() {
    let mut bv = SimpleBitVector::new(32, false);

    assert!(bv.try_set(32, true) == false);
}

#[test]
fn try_get_index_out_of_bounds() {
    let bv = SimpleBitVector::new(32, false);
    let value = bv.try_get(32);
    assert!(value.is_none());
}

#[test]
fn consumable_iterator() {
    let bv = SimpleBitVector::new(5, false);
    let mut accumulated_result: Vec<bool> = Vec::new();
    for item in bv {
        accumulated_result.push(item);
    }
    assert!(
        accumulated_result == vec![false, false, false, false, false],
        "Iterator didn't return valid result"
    );

    // when we try to access bv here, we get error that the value has already moved.

    // let value = bv.try_get(0);
}

#[test]
fn non_consumable_iterator() {
    let mut bv = SimpleBitVector::new(5, false);
    let mut accumulated_result: Vec<bool> = Vec::new();
    for item in &bv {
        accumulated_result.push(item);
    }
    assert!(
        accumulated_result == vec![false, false, false, false, false],
        "Iterator didn't return valid result"
    );

    // when we try to access bv here, we can still access it.
    // ACT
    let _ = bv.try_get(0);
    let _ = bv.try_set(0, true);

    accumulated_result.clear();
    for item in &bv {
        accumulated_result.push(item);
    }

    // ASSERT
    assert!(
        accumulated_result == vec![true, false, false, false, false],
        "Iterator didn't return valid result"
    );
}
