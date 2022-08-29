#![allow(clippy::not_unsafe_ptr_arg_deref)]
pub mod plugin;
use plugin::{add_import_extension, config::Config};

use swc_core::{
    ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    visit::FoldWith,
};

#[plugin_transform]
pub fn add_import_extension_plugin(
    program: Program,
    data: TransformPluginProgramMetadata,
) -> Program {
    let sources = serde_json::from_str(
        &data
            .get_transform_plugin_config()
            .expect("add-import-extension: failed to get plugin config"),
    )
    .expect("add-import-extension: invalid plugin config");

    program.fold_with(&mut add_import_extension(Config { sources }))
}
