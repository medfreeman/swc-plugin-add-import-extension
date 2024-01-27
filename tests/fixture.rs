mod utils;
use utils::typescript_transform;

use std::path::PathBuf;

use swc_plugin_add_import_extension::plugin::{
    add_import_extension,
    config::{Config, SourceConfig},
};

use swc_core::{
    common::chain,
    ecma::parser::{Syntax, TsConfig},
    ecma::transforms::testing::test_fixture,
};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Typescript(TsConfig {
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.js")]
#[fixture("tests/fixture/typescript/**/input.ts")]
fn add_import_extension_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_t| {
            chain!(
                typescript_transform(),
                add_import_extension(Config {
                    sources: vec![(
                        "^\\..*".to_string(),
                        SourceConfig {
                            extension: "mjs".into(),
                            add_index: true,
                        },
                    )]
                    .into_iter()
                    .collect(),
                }),
            )
        },
        &input,
        &output,
        swc_core::ecma::transforms::testing::FixtureTestConfig::default(),
    );
}
