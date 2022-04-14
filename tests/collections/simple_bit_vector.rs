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
