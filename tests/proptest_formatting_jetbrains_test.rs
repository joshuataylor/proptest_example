#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {
#![proptest_config(ProptestConfig::with_cases(2))]
#[test]
fn test_example(a in 1..4i32, b in 5..10i32) {
prop_assert!(a <= b);
}
}
}
