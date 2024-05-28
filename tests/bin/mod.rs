
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub(crate) struct Output {
    pub(crate) success: bool,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stderr: Vec<u8>,
}

pub(crate) fn run(
    compiled: &str,
    unsafe_main: unsafe fn(
        stdin: &mut dyn Read,
        stdout: &mut dyn Write,
    ) -> Result<(), Box<dyn Error>>,
    input: &Path,
) -> Output {
    if cfg!(miri) {
        let mut input = File::open(input).unwrap();
        let mut stdout = Vec::new();
        let result = unsafe { unsafe_main(&mut input, &mut stdout) };

        Output {
            success: result.is_ok(),
            stdout,
            stderr: result
                .err()
                .as_ref()
                .map_or_else(String::new, ToString::to_string)
                .into(),
        }
    } else {
        let output = Command::new(compiled)
            .arg(input)
            .stdin(Stdio::null())
            .output()
            .unwrap();

        Output {
            success: output.status.success(),
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}
