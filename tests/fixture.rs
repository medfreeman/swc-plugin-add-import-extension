use std::path::PathBuf;

use swc_plugin_add_import_extension::plugin::{
    add_import_extension,
    config::{Config, SourceConfig},
};

use swc_core::{
    ecma::parser::{EsConfig, Syntax},
    ecma::transforms::testing::test_fixture,
};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.js")]
fn add_import_extension_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_tr| {
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
            })
        },
        &input,
        &output,
        swc_core::ecma::transforms::testing::FixtureTestConfig::default(),
    );
}
