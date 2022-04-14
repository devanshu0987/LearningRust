use common::collections::fixed_size_array_pool::{FixedSizeArrayPool, Response};

#[test]
fn it_works() {
    let mut pool = FixedSizeArrayPool::<u32, 5, 5>::new();
    assert_eq!(pool.len(), 5);
    let mut buffer = pool.acquire();
    assert_eq!(pool.len(), 4);
    // clean up before use. This would be linear in time.
    // TODO: make this faster.
    // I wonder, if we add one more element along with each segment to indicate
    // I have written till this index, don't read post this.
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

#[test]
fn it_works_v2() {
    let mut pool = FixedSizeArrayPool::<u8, 5, 5>::new();
    let buffer = pool.acquire();
    assert_eq!(pool.len(), 4);
    // release
    match pool.release(buffer) {
        Response::PoolSaturated => assert!(false, "Release failed because pool is full"),
        _ => assert!(true),
    }

    assert_eq!(pool.len(), 5);

    // it doesn't let me push in any other size buffer back
    // that test passes at compile time
    let poison_buffer = [0u8; 5];
    match pool.release(poison_buffer) {
        Response::PoolSaturated => assert!(true, "Release failed because pool is full"),
        _ => assert!(false),
    }
}
