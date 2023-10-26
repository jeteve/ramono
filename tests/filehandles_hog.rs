use test_log::test;
// To see the logs, run with:
// $ RUST_LOG=debug cargo test   -- --nocapture


use ramono::FileHandlesHog;


#[test]
fn simple_test(){
    let mut hog = FileHandlesHog::new(10, 20);
    hog.tick();
    assert!(! hog.saturated() );
    hog.tick();
    assert!( hog.saturated() );
}

#[test]
fn with_error(){
    let mut hog = FileHandlesHog::new(1000,  100000);
    for _ in 0..100{
        hog.tick();
    }
    assert!( hog.saturated() );
}