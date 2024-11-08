use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("poet")?;

    cmd.arg("-f").arg("foo").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A fox\njumped over\na foo\nthen a bar\nthen a foobar")?;

    let mut cmd = Command::cargo_bin("poet")?;
    cmd.arg("-f").arg("foo").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a foo\nthen a foobar"));

    Ok(())
}
