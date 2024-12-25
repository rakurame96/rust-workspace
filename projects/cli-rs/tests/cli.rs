#[test]
fn works() {
    assert!(true);
}

#[test]
fn not_works() {
    // assert!(false);         // this fails and uncomment to observe
    assert_eq!(false, false);
}

#[test]
fn runs_ls() {
    use std::process::Command;

    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn runs_cli_rs() {
    use std::process::Command;

    let mut cmd = Command::new("cli-rs");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn runs_bin_dir() {
    use assert_cmd::Command;

    // let mut cmd = Command::cargo_bin("hello").unwrap();        // fails as there is no crate named 'hello'
    let mut cmd = Command::cargo_bin("cli-rs").unwrap();
    cmd.assert().success();
}

#[test]
fn true_ok() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure(); 
}

#[test]
fn runs_bin_dir_stdout() {
    use assert_cmd::Command;

    // let mut cmd = Command::cargo_bin("hello").unwrap();        // fails as there is no crate named 'hello'
    let mut cmd = Command::cargo_bin("cli-rs").unwrap();
    // cmd.assert().success().stdout("Hello, World!\n");       // fails as the output is "Hello, world!\n"
    cmd.assert().success().stdout("Hello, world!\n");
}