use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};
use poise::serenity_prelude::GuildId;
use crate::config::ServerConfig;

pub struct App {
    server_configs: RwLock<HashMap<GuildId, Arc<ServerConfig>>>
}

impl App {
    pub fn new() -> Self {
        App {
            server_configs: RwLock::new(HashMap::new())
        }
    }

    pub fn get_config(&self, id: GuildId) -> Arc<ServerConfig> {
        if let Some(config) = self.server_configs.read().unwrap().get(&id) {
            return Arc::clone(config);
        }

        let config = Arc::new(
            knus::parse("<config>", &fs::read_to_string(format!("./configs/{id}.kdl")).unwrap())
                .map_err(miette::Report::new)
                .unwrap(),
        );

        self.server_configs
            .write()
            .unwrap()
            .entry(id)
            .or_insert_with(|| Arc::clone(&config))
            .clone()
    }
}
