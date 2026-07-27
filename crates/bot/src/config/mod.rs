use std::collections::HashSet;
use std::hash::Hash;
use std::mem::discriminant;

#[derive(knus::Decode, Debug)]
pub struct ServerConfig {
    #[knus(child, unwrap(argument), default = "!".to_string())]
    pub prefix: String,
    #[knus(child, unwrap(children), default = HashSet::new())]
    pub levels: HashSet<PermissionLevel>,
    #[knus(child, unwrap(children), default = HashSet::new())]
    pub plugins: HashSet<Plugins>,
}

#[derive(knus::Decode, Debug)]
pub struct PermissionLevel {
    #[knus(node_name)]
    pub role_id: String,
    #[knus(argument)]
    pub value: i64,
}

impl PartialEq for PermissionLevel {
    fn eq(&self, other: &Self) -> bool {
        self.role_id == other.role_id
    }
}

impl Eq for PermissionLevel {}

impl Hash for PermissionLevel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.role_id.hash(state);
    }
}

#[derive(knus::Decode, Debug)]
pub enum Plugins {
    ModActions(Plugin<ModActionsConfig>),
}

impl PartialEq for Plugins {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for Plugins {}

impl Hash for Plugins {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}

#[derive(knus::Decode, Debug)]
struct Plugin<T: knus::Decode> {
    #[knus(child)]
    pub config: T,
    #[knus(child, unwrap(children(name = "override")))]
    pub overrides: Vec<PluginOverride<T>>,
}

#[derive(knus::Decode, Debug)]
struct PluginOverride<T: knus::Decode> {
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
    use crate::config::ServerConfig;

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

        let parsed_config: ServerConfig = knus::parse("<test>", config_str)
            .map_err(miette::Report::new)
            .unwrap();

        dbg!(&parsed_config);

        assert_eq!(parsed_config.prefix, "!");
        assert_eq!(parsed_config.levels.len(), 2);
    }
}
