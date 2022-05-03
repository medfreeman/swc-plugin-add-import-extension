use once_cell::sync::Lazy;
use regex::Regex;
use swc_plugin::{ast::*, plugin_transform, TransformPluginProgramMetadata};
use tracing::info;

static MJS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.*?(?P<ext>\.mjs)?$").unwrap());

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, imp_declaration: &mut ImportDecl) {
        imp_declaration.visit_mut_children_with(self);

        info!(
            "visiting import declaration, source: {}",
            &imp_declaration.src.value
        );

        let group = MJS_REGEX.captures(&imp_declaration.src.value);
        if let Some(group) = group {
            if group.name("ext") == None {
                info!("match, source: {:?}", &group[0]);
                imp_declaration.src =
                    swc_plugin::ast::Str::from(format!("{}.mjs", &imp_declaration.src.value));
            }
        }
    }

    fn visit_mut_import_specifier(&mut self, imp_specifier: &mut ImportSpecifier) {
        imp_specifier.visit_mut_children_with(self);
    }
}

#[plugin_transform]
pub fn add_mjs_ext_plugin(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

#[cfg(test)]
mod transform_visitor_tests {
    use swc_ecma_transforms_testing::test;

    use super::*;

    fn transform_visitor() -> impl 'static + Fold + VisitMut {
        as_folder(TransformVisitor)
    }

    test!(
        ::swc_ecma_parser::Syntax::default(),
        |_| transform_visitor(),
        add_mjs_ext_when_source_is_relative,
        r#"import { test } from "./entry";"#,
        r#"import { test } from "./entry.mjs";"#
    );

    test!(
        ::swc_ecma_parser::Syntax::default(),
        |_| transform_visitor(),
        doesnt_add_mjs_ext_when_source_already_has_it,
        r#"import { test } from "./entry.mjs";"#,
        r#"import { test } from "./entry.mjs";"#
    );
}
