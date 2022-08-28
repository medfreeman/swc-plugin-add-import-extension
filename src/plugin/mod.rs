#![allow(dead_code)]
pub mod config;
use config::{Config, SourceConfig};

mod rewriter;
use rewriter::Rewriter;

use swc_cached::regex::CachedRegex;
use swc_core::ast::*;
use swc_core::visit::{noop_fold_type, Fold};

struct FoldImports {
    sources: Vec<(CachedRegex, SourceConfig)>,
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
                            new_items.push(ModuleItem::ModuleDecl(ModuleDecl::Import(rewritten)));
                        }
                        None => new_items.push(ModuleItem::ModuleDecl(ModuleDecl::Import(decl))),
                    },
                    ModuleDecl::ExportNamed(decl) => match &decl.src {
                        Some(src) => match self.should_rewrite(&src.value) {
                            Some(rewriter) => {
                                let rewritten = rewriter.rewrite_export(&decl);
                                new_items.push(ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(
                                    rewritten,
                                )));
                            }
                            None => new_items
                                .push(ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(decl))),
                        },
                        None => {
                            new_items.push(ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(decl)))
                        }
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
