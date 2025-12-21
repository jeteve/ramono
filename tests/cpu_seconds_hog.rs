use std::time::Instant;

use ramono::CpuSecondsHog;

#[test]
fn zero_test() {
    let mut hog = CpuSecondsHog::new(0, 2);
    hog.tick();
    assert!(!hog.saturated());
    hog.tick();
    assert!(!hog.saturated());
    drop(hog);
}

#[test]
fn non_zero_test() {
    let start = Instant::now();
    let mut hog = CpuSecondsHog::new(1, 2);
    hog.tick();
    assert!(!hog.saturated());
    hog.tick();
    assert!(hog.saturated());
    drop(hog);
    assert!(start.elapsed().as_secs() >= 2);
}
