use crate::config::ModuleConfig;
use serde::Serialize;
use starship_module_config_derive::ModuleConfig;
use std::collections::HashMap;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct AwsConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub region_aliases: HashMap<String, &'a str>,
    pub profile_aliases: HashMap<String, &'a str>,
    pub expiration_symbol: &'a str,
    pub force_display: bool,
}

impl<'a> Default for AwsConfig<'a> {
    fn default() -> Self {
        AwsConfig {
            format: "on [$symbol($profile )(\\($region\\) )(\\[$duration\\])]($style)",
            symbol: "☁️  ",
            style: "bold yellow",
            disabled: false,
            region_aliases: HashMap::new(),
            profile_aliases: HashMap::new(),
            expiration_symbol: "X",
            force_display: false,
        }
    }
}
