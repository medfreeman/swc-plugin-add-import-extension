use std::collections::HashMap;

use serde::Deserialize;

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
