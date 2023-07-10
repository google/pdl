// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Various utility functions used in tests.

// This file is included directly into integration tests in the
// `tests/` directory. These tests are compiled without access to the
// rest of the `pdl` crate. To make this work, avoid `use crate::`
// statements below.

use googletest::prelude::{assert_that, eq};
use std::fs;
use std::path::Path;

/// Format Rust code in `input`.
pub fn format_rust(input: &str) -> String {
    let syntax_tree = syn::parse_file(input).expect("Could not parse {input:#?} as Rust code");
    let formatted = prettyplease::unparse(&syntax_tree);
    format!("#![rustfmt::skip]\n{formatted}")
}

/// Compare a string with a snapshot file.
///
/// The `snapshot_path` is relative to the current working directory
/// of the test binary. This depends on how you execute the tests:
///
/// * When using `atest`: The current working directory is a random
///   temporary directory. You need to ensure that the snapshot file
///   is installed into this directory. You do this by adding the
///   snapshot to the `data` attribute of your test rule
///
/// * When using Cargo: The current working directory is set to
///   `CARGO_MANIFEST_DIR`, which is where the `Cargo.toml` file is
///   found.
///
/// If you run the test with Cargo and the `UPDATE_SNAPSHOTS`
/// environment variable is set, then the `actual_content` will be
/// written to `snapshot_path`. Otherwise the content is compared and
/// a panic is triggered if they differ.
#[track_caller]
pub fn assert_snapshot_eq<P: AsRef<Path>>(snapshot_path: P, actual_content: &str) {
    let update_snapshots = std::env::var("UPDATE_SNAPSHOTS").is_ok();
    let snapshot = snapshot_path.as_ref();
    let snapshot_content = match fs::read(snapshot) {
        Ok(content) => content,
        Err(_) if update_snapshots => Vec::new(),
        Err(err) => panic!("Could not read snapshot from {}: {}", snapshot.display(), err),
    };
    let snapshot_content = String::from_utf8(snapshot_content).expect("Snapshot was not UTF-8");

    // Normal comparison if UPDATE_SNAPSHOTS is unset.
    if !update_snapshots {
        assert_that!(actual_content, eq(&snapshot_content));
    }

    // Bail out if we are not using Cargo.
    if std::env::var("CARGO_MANIFEST_DIR").is_err() {
        panic!("Please unset UPDATE_SNAPSHOTS if you are not using Cargo");
    }

    if actual_content != snapshot_content {
        eprintln!(
            "Updating snapshot {}: {} -> {} bytes",
            snapshot.display(),
            snapshot_content.len(),
            actual_content.len()
        );
        fs::write(&snapshot_path, actual_content).unwrap_or_else(|err| {
            panic!("Could not write snapshot to {}: {}", snapshot.display(), err)
        });
    }
}
