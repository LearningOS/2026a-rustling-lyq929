
#[test]
fn test_feature() {
    #[cfg(feature = "pass")]
    {
        return;
    }
    panic!("You should pass the --cfg 'feature=\"pass\"' argument to rustc.");
}
