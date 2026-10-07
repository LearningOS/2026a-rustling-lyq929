
#[test]
fn test_env_var() {
    let foo = std::env::var("TEST_FOO").unwrap();
    println!("{}", foo);
}
