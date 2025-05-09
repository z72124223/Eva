use eva_tools::fine_tune;

#[test]
fn test_run_nightly_fine_tune() {
    let result = fine_tune::run_nightly_fine_tune();
    assert!(result.is_ok());
}
