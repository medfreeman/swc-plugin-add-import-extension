#![allow(dead_code)]
use swc_core::{
    common::{chain, pass::Optional, Mark},
    ecma::{
        transforms::{
            base::resolver,
            compat::es2022::class_properties,
            proposal::decorators,
            testing::Tester,
            typescript,
            typescript::{
                typescript as strip_with_config, ImportsNotUsedAsValues, TsImportExportAssignConfig,
            },
        },
        visit::Fold,
    },
};

pub fn typescript_transform() -> impl Fold {
    typescript_transform_config(None, None)
}

fn typescript_transform_config(
    config: Option<typescript::Config>,
    decorators_config: Option<decorators::Config>,
) -> impl Fold {
    let mark = Mark::fresh(Mark::root());
    let has_decorators = decorators_config.is_some();
    let config = config.unwrap_or_else(|| typescript::Config {
        import_export_assign_config: TsImportExportAssignConfig::EsNext,
        import_not_used_as_values: ImportsNotUsedAsValues::Preserve,
        no_empty_export: true,
        verbatim_module_syntax: true,
        ..Default::default()
    });
    chain!(
        Optional::new(
            decorators(decorators_config.unwrap_or_default()),
            has_decorators,
        ),
        resolver(Mark::new(), mark, true),
        strip_with_config(config, mark),
    )
}

pub fn properties(t: &Tester, loose: bool) -> impl Fold {
    class_properties(
        Some(t.comments.clone()),
        class_properties::Config {
            set_public_fields: loose,
            ..Default::default()
        },
        Mark::fresh(Mark::default()),
    )
}
