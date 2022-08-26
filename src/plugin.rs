#![allow(dead_code)]

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;
use swc_cached::regex::CachedRegex;
use swc_core::ast::*;
use swc_core::visit::{noop_fold_type, Fold};

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct Config {
    pub sources: HashMap<String, SourceConfig>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceConfig {
    pub extension: String,
    #[serde(default)]
    pub add_index: bool,
}

struct FoldImports {
    sources: Vec<(CachedRegex, SourceConfig)>,
}

struct Rewriter<'a> {
    key: &'a str,
    config: &'a SourceConfig,
    src: &'a str,
}

impl<'a> Rewriter<'a> {
    fn rewrite_import(&self, old_decl: &ImportDecl) -> Vec<ImportDecl> {
        if old_decl.type_only || old_decl.asserts.is_some() {
            return vec![old_decl.clone()];
        }

        let mut out: Vec<ImportDecl> = Vec::with_capacity(old_decl.specifiers.len());

        for spec in &old_decl.specifiers {
            match spec {
                ImportSpecifier::Named(named_spec) => {
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

                    let specifier = ImportSpecifier::Named(named_spec.clone());

                    out.push(ImportDecl {
                        specifiers: vec![specifier],
                        src: Str::from(new_path.as_ref()),
                        span: old_decl.span,
                        type_only: false,
                        asserts: None,
                    });
                }
                _ => {
                    // Give up
                    return vec![old_decl.clone()];
                }
            }
        }

        out
    }
}

impl FoldImports {
    fn should_rewrite<'a>(&'a self, name: &'a str) -> Option<Rewriter<'a>> {
        for (regex, config) in &self.sources {
            let group = regex.captures(name);
            if let Some(group) = group {
                let src = group.get(0).map_or("", |m| m.as_str());

                return Some(Rewriter {
                    key: name,
                    config,
                    src,
                });
            }
        }

        None
    }
}

impl Fold for FoldImports {
    noop_fold_type!();

    fn fold_module(&mut self, mut module: Module) -> Module {
        let mut new_items: Vec<ModuleItem> = vec![];

        for item in module.body {
            match item {
                ModuleItem::ModuleDecl(module_decl) => match module_decl {
                    ModuleDecl::Import(decl) => match self.should_rewrite(&decl.src.value) {
                        Some(rewriter) => {
                            let rewritten = rewriter.rewrite_import(&decl);
                            new_items.extend(
                                rewritten
                                    .into_iter()
                                    .map(|x| ModuleItem::ModuleDecl(ModuleDecl::Import(x))),
                            );
                        }
                        None => new_items.push(ModuleItem::ModuleDecl(ModuleDecl::Import(decl))),
                    },
                    x => {
                        new_items.push(ModuleItem::ModuleDecl(x));
                    }
                },
                x => {
                    new_items.push(x);
                }
            }
        }

        module.body = new_items;

        module
    }
}

pub fn add_import_extension(config: Config) -> impl Fold {
    let mut folder = FoldImports { sources: vec![] };

    for (k, v) in config.sources {
        folder.sources.push((
            CachedRegex::new(&k).expect("add-import-extension: invalid regex"),
            v,
        ));
    }

    folder
}
