use vertex_dev_release_guard::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 54, capacity: 71, latency: 8, risk: 20, weight: 11 };
    assert_eq!(score(signal), 60);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 64, capacity: 82, latency: 21, risk: 18, weight: 6 };
    assert_eq!(score(signal), 36);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 79, capacity: 72, latency: 21, risk: 12, weight: 10 };
    assert_eq!(score(signal), 104);
    assert_eq!(classify(signal), "review");
}
