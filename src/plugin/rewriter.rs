use std::path::Path;

use super::config::SourceConfig;

use swc_core::ast::*;

pub struct Rewriter<'a> {
    key: &'a str,
    config: &'a SourceConfig,
    src: &'a str,
}

impl<'a> Rewriter<'a> {
    pub fn new(key: &'a str, config: &'a SourceConfig, src: &'a str) -> Rewriter<'a> {
        Rewriter { key, config, src }
    }

    fn rewrite_src(&self) -> Str {
        let path = Path::new(self.src);

        let new_path = format!(
            "{}{}.{}",
            path.parent()
                .unwrap()
                .join(path.file_stem().unwrap())
                .to_string_lossy(),
            if self.config.add_index && path.extension() == None {
                "/index"
            } else {
                ""
            },
            self.config.extension
        );

        Str::from(new_path.as_ref())
    }

    pub fn rewrite_import(&self, old_decl: &ImportDecl) -> ImportDecl {
        if old_decl.type_only || old_decl.asserts.is_some() {
            return old_decl.clone();
        }

        ImportDecl {
            specifiers: old_decl.specifiers.clone(),
            src: self.rewrite_src(),
            span: old_decl.span,
            type_only: false,
            asserts: None,
        }
    }

    pub fn rewrite_named_export(&self, old_decl: &NamedExport) -> NamedExport {
        if old_decl.type_only || old_decl.asserts.is_some() {
            return old_decl.clone();
        }

        NamedExport {
            specifiers: old_decl.specifiers.clone(),
            src: Option::from(self.rewrite_src()),
            span: old_decl.span,
            type_only: false,
            asserts: None,
        }
    }

    pub fn rewrite_export_all(&self, old_decl: &ExportAll) -> ExportAll {
        if old_decl.asserts.is_some() {
            return old_decl.clone();
        }

        ExportAll {
            src: self.rewrite_src(),
            span: old_decl.span,
            asserts: None,
        }
    }
}
