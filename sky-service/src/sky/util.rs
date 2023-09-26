fn assert_epsilon_eq(is: f64, want: f64, eps: f64) {
    let diff = (is - want).abs();
    assert!(
        diff < eps,
        "the difference of {is} (is) and {want} (want) is {diff}, which is larger than {eps} (epsilon)",
    )
}

pub fn assert_approx_eq(is: f64, want: f64) {
    assert_epsilon_eq(is, want, 5e-5)
}
