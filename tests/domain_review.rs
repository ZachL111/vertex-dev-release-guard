use vertex_dev_release_guard::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 78, slack: 25, drag: 30, confidence: 92 };
    assert_eq!(review_score(case), 183);
    assert_eq!(review_lane(case), "ship");
}
