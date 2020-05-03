use common::util::*;

#[test]
fn test_df_compatible() {
    let (_, mut ucmd) = at_and_ucmd!();
    let result = ucmd.arg("-ah").run();
    assert!(result.success);
}

#[test]
fn test_df_compatible_type() {
    let (_, mut ucmd) = at_and_ucmd!();
    let result = ucmd.arg("-aT").run();
    assert!(result.success);
}
// TODO
