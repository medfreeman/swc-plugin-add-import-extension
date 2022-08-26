pub mod plugin;
use plugin::{add_import_extension, Config};

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
            .expect("failed to get plugin config for add-import-extension"),
    )
    .expect("invalid config");

    program.fold_with(&mut add_import_extension(Config { sources }))
}
