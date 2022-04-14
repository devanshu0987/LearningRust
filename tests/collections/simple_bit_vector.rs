use common::collections::simple_bit_vector::SimpleBitVector;
#[test]
fn new_works() {
    assert!(SimpleBitVector::new(32, false).length == 1);
    assert!(SimpleBitVector::new(31, true).length == 1);
    assert!(SimpleBitVector::new(1, true).length == 1);
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
}

#[test]
fn set_get_multiple_success() {
    let mut bv = SimpleBitVector::new(32, false);
    for index in &[1u32..5] {
        assert!(bv.get((*index).start) == false);
        bv.set((*index).start, true);
    }

    for index in &[1u32..5] {
        assert!(bv.get((*index).start) == true);
    }
}

#[test]
fn set_fails(){
    let result = std::panic::catch_unwind(|| {
        let mut bv = SimpleBitVector::new(32, false);
        bv.set(32, true);
    });
    assert!(result.is_err());
}

#[test]
fn get_fails(){
    let result = std::panic::catch_unwind(|| {
        let bv = SimpleBitVector::new(32, false);
        bv.get(32);
    });
    assert!(result.is_err());
}