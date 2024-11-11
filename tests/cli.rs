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
    file.write_str("A foo\njumped over a bar\nthen a foobar")?;

    let mut cmd = Command::cargo_bin("poet")?;
    cmd.arg("-f").arg("foo").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::eq("A foo\nthen a foobar\n"));

    Ok(())
}

#[test]
fn print_all_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A foo\njumped over a bar\nthen a foobar")?;

    let mut cmd = Command::cargo_bin("poet")?;
    cmd.arg("-a").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::eq("A foo\njumped over a bar\nthen a foobar\n"));

    Ok(())
}

#[test]
fn replace_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A foo\njumped over a bar\nthen a foobar")?;

    let mut cmd = Command::cargo_bin("poet")?;
    cmd.arg("-r").arg("foo").arg("baz").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::eq("A baz\nthen a bazbar\n"));

    Ok(())
}

#[test]
fn unexpected_argument() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A foo\njumped over a bar\nthen a foobar")?;

    let mut cmd = Command::cargo_bin("poet")?;
    cmd.arg("--foo").arg(file.path());
    cmd.assert().failure().stderr(predicate::str::contains(
        "unexpected argument '--foo' found",
    ));

    Ok(())
}

#[test]
fn print_all_content_when_no_option() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A foo\njumped over a bar\nthen a foobar")?;

    let mut cmd = Command::cargo_bin("poet")?;
    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::eq("A foo\njumped over a bar\nthen a foobar\n"));

    Ok(())
}

#[test]
fn find_and_unfold_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A foo\njumped over a bar\nthen a foobar")?;

    let frens = assert_fs::NamedTempFile::new("frens.txt")?;
    frens.write_str("cat\ndog")?;

    let mut cmd = Command::cargo_bin("poet")?;
    cmd.arg("-f")
        .arg("foo")
        .arg(file.path())
        .arg("-u")
        .arg("-i")
        .arg(frens.path());
    cmd.assert().success().stdout(predicate::eq(
        "A cat\nA dog\nthen a catbar\nthen a dogbar\n",
    ));

    Ok(())
}
