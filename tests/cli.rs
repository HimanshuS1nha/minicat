use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[test]
fn reports_file_errors_without_panicking() {
    let files = InputFiles::new();
    let missing = files.0.join("missing.txt");
    let invalid = files.0.join("invalid.txt");
    fs::write(&invalid, [0xff]).unwrap();

    for path in [missing, invalid] {
        let output = Command::new(env!("CARGO_BIN_EXE_minicat"))
            .arg(&path)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(error.starts_with("minicat: "), "{error}");
        assert!(error.contains(path.to_str().unwrap()), "{error}");
        assert!(!error.contains("panicked"), "{error}");
    }
}

struct InputFiles(PathBuf);

impl InputFiles {
    fn new() -> Self {
        let directory = std::env::temp_dir().join(format!(
            "minicat-test-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&directory).unwrap();
        Self(directory)
    }

    fn run(&self, flags: &[&str], inputs: &[&[u8]]) -> Vec<u8> {
        let mut command = Command::new(env!("CARGO_BIN_EXE_minicat"));
        command.args(flags);
        for (index, contents) in inputs.iter().enumerate() {
            let path = self.0.join(format!("{index}.txt"));
            fs::write(&path, contents).unwrap();
            command.arg(path);
        }
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "flags {flags:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        output.stdout
    }
}

impl Drop for InputFiles {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn concatenates_files_without_changing_contents() {
    let files = InputFiles::new();
    for inputs in [
        vec![b"".as_slice(), b"hello\n", b"\nworld\n"],
        vec![b"hello".as_slice(), b"world"],
        vec![b"hello\r\n".as_slice()],
        vec![b"\r\n\nhello\rworld\nlast\r".as_slice()],
    ] {
        assert_eq!(files.run(&[], &inputs), inputs.concat(), "{inputs:?}");
    }
}

#[test]
fn numbers_lines_across_files_with_nonblank_taking_precedence() {
    let files = InputFiles::new();
    let inputs: &[&[u8]] = &[b"\nhello\n\n", b" \n\t\nworld\n"];
    let all = b"1 \n2 hello\n3 \n4  \n5 \t\n6 world\n";
    let nonblank = b"\n1 hello\n\n2  \n3 \t\n4 world\n";
    for (flags, expected) in [
        (vec!["-n"], all.as_slice()),
        (vec!["--number"], all.as_slice()),
        (vec!["-b"], nonblank.as_slice()),
        (vec!["--number-nonblank"], nonblank.as_slice()),
        (vec!["-n", "-b"], nonblank.as_slice()),
        (vec!["-b", "-n"], nonblank.as_slice()),
    ] {
        assert_eq!(files.run(&flags, inputs), expected, "flags {flags:?}");
    }
}

#[test]
fn displays_tabs_and_line_ends() {
    let files = InputFiles::new();
    let inputs: &[&[u8]] = &[b"a\tb\\t\n\n"];
    for (flags, expected) in [
        (vec!["-T"], "a^Ib\\t\n\n"),
        (vec!["--show-tabs"], "a^Ib\\t\n\n"),
        (vec!["-E"], "a\tb\\t$\n$\n"),
        (vec!["--show-ends"], "a\tb\\t$\n$\n"),
        (vec!["-T", "-E"], "a^Ib\\t$\n$\n"),
        (vec!["-A"], "a^Ib\\t$\n$\n"),
        (vec!["--show-all"], "a^Ib\\t$\n$\n"),
        (vec!["-b", "-A"], "1 a^Ib\\t$\n$\n"),
    ] {
        assert_eq!(
            files.run(&flags, inputs),
            expected.as_bytes(),
            "flags {flags:?}"
        );
    }

    for (flags, expected) in [
        (vec!["-E"], "a\tb"),
        (vec!["-A"], "a^Ib"),
        (vec!["-n"], "1 a\tb"),
    ] {
        assert_eq!(
            files.run(&flags, &[b"a\tb"]),
            expected.as_bytes(),
            "unterminated line with flags {flags:?}"
        );
    }
}
