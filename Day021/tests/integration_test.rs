// need to bring the modules into scope to be utilized here
// we do not need to use cfg attribute here because in tests folder are for testing purpose only
// to onlu run the integration tests, we can use `cargo test --test integration_test`

mod  common;

#[test]
fn some_integration_test() {
    common::setup();
    // This is a placeholder for an integration test
    assert!(true, "This is a dummy integration test");
}