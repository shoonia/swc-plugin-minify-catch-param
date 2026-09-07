mod setup;

use crate::setup::{syntax, visitor};
use std::path::PathBuf;
use swc_core::ecma::transforms::testing::{test_fixture, FixtureTestConfig};

#[testing::fixture("tests/fixture/**/input.js")]
fn transformer_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        syntax(),
        &|_| visitor(),
        &input,
        &output,
        FixtureTestConfig {
            allow_error: true,
            module: Some(true),
            ..Default::default()
        },
    );
}
