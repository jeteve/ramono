use ramono::MemoryHog;

#[test]
fn simple_test(){
    let mut hog = MemoryHog::new(10, 20);
    hog.tick();
    assert!(! hog.saturated() );
    hog.tick();
    assert!( hog.saturated() );
}