// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibYML. All rights reserved.

#![allow(clippy::type_complexity, clippy::uninlined_format_args)]

mod bin;
#[path = "../src/bin/run-parser-test-suite.rs"]
#[allow(dead_code)]
mod run_parser_test_suite;

use std::path::Path;

fn test(id: &str) {
    let dir = Path::new("tests")
        .join("data")
        .join("yaml-test-suite")
        .join(id);

    let output = bin::run(
        env!("CARGO_BIN_EXE_run-parser-test-suite"),
        run_parser_test_suite::unsafe_main,
        &dir.join("in.yaml"),
    );

    if output.success {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprint!("{}", stdout);
        eprint!("{}", stderr);
        panic!("expected parse to fail");
    }
}

libyml_test_suite::test_parser_error!();
