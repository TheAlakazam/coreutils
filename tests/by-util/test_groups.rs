use crate::common::util::*;

#[test]
fn test_groups() {
    let result = new_ucmd!().run();
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    if is_ci() && result.stdout_str().trim().is_empty() {
        // In the CI, some server are failing to return the group.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    result.success();
    assert!(!result.stdout_str().trim().is_empty());
}

#[test]
fn test_groups_arg() {
    // get the username with the "id -un" command
    let result = TestScenario::new("id").ucmd_keepenv().arg("-un").run();
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    let s1 = String::from(result.stdout_str().trim());
    if is_ci() && s1.parse::<f64>().is_ok() {
        // In the CI, some server are failing to return id -un.
        // So, if we are getting a uid, just skip this test
        // As seems to be a configuration issue, ignoring it
        return;
    }

    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    result.success();
    assert!(!result.stdout_str().is_empty());
    let username = result.stdout_str().trim();

    // call groups with the user name to check that we
    // are getting something
    new_ucmd!().arg(username).succeeds();
    assert!(!result.stdout_str().is_empty());
}
