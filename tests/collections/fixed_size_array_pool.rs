use common::collections::fixed_size_array_pool::{FixedSizeArrayPool, Response};

#[test]
fn it_works() {
    let mut pool = FixedSizeArrayPool::<u32, 5, 5>::new();
    assert_eq!(pool.len(), 5);
    let mut buffer = pool.acquire();
    assert_eq!(pool.len(), 4);
    // clean up before use. This would be linear in time.
    // I don't know how to make this faster.
    // I wonder, if we add one more element as wrote till here with every one,
    // then we can get away with this
    buffer.fill(u32::default());
    // use it for something
    buffer[0] = 1u32;
    println!("{:?}", buffer);
    // release
    match pool.release(buffer) {
        Response::PoolSaturated => assert!(false, "Release failed because pool is full"),
        _ => assert!(true),
    }

    assert_eq!(pool.len(), 5);
}
