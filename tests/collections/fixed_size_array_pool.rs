use common::collections::fixed_size_array_pool::FixedSizeArrayPool;

#[test]
fn it_works() {
    let mut pool = FixedSizeArrayPool::new(5, 5);
    assert_eq!(pool.len(), 5);
    let mut buffer = pool.acquire();
    assert_eq!(pool.len(), 4);
    // clean up before use. This would be linear in time
    buffer.fill(0);
    // use it for something
    buffer[0] = 1;
    println!("{:?}", buffer);
    // release
    let status = pool.release(buffer);
    assert_eq!(status, true);
    assert_eq!(pool.len(), 5);
}
