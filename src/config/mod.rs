#[derive(knus::Decode, Debug)]
pub struct BotConfig {
    #[knus(child, unwrap(argument), default = "!".to_string())]
    pub prefix: String,
    #[knus(child, unwrap(children))]
    pub levels: Vec<PermissionLevel>,
    #[knus(child, unwrap(children))]
    pub plugins: Vec<Plugins>,
}

#[derive(knus::Decode, Debug)]
struct PermissionLevel {
    #[knus(node_name)]
    pub role_id: String,
    #[knus(argument)]
    pub value: i64,
}

#[derive(knus::Decode, Debug)]
enum Plugins {
    ModActions(Plugin<ModActionsConfig>),
}

#[derive(knus::Decode, Debug)]
struct Plugin<T> {
    #[knus(child)]
    pub config: T,
    #[knus(child, unwrap(children(name = "override")))]
    pub overrides: Vec<PluginOverride<T>>,
}

#[derive(knus::Decode, Debug)]
struct PluginOverride<T> {
    #[knus(argument)]
    pub level: String,
    #[knus(child)]
    pub config: T,
}

#[derive(knus::Decode, Debug)]
struct ModActionsConfig {
    #[knus(child, unwrap(argument), default = Option::Some("You have been kicked from the {guildName} server. Reason given: {reason}".into()))]
    pub kick_message: Option<String>,
    #[knus(child, unwrap(argument), default = Option::None)]
    pub can_kick: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::config::BotConfig;

    #[test]
    fn test_config_decode() {
        let config_str = r#"
            prefix "!"

            levels {
                "172949857164722176" 100 // Example admin
                "172950000412655616" 50  // Example mod
            }

            plugins {
                mod-actions {
                    config {
                        kick-message "You have been kicked"
                        can-kick #true
                    }

                    overrides {
                        override ">=50" {
                            config {
                                can-kick #true
                            }
                        }

                        override ">=100" {
                            config {
                                kick-message "You have been kicked by an admin"
                            }
                        }
                    }
                }
            }"#;

        let parsed_config: BotConfig = knus::parse("<test>", config_str)
            .map_err(miette::Report::new)
            .unwrap();

        dbg!(&parsed_config);

        assert_eq!(parsed_config.prefix, "!");
        assert_eq!(parsed_config.levels.len(), 2);
    }
}
