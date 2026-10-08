use crate::config::ServerConfig;
use database_entity::allowed_guilds;
use database_entity::prelude::AllowedGuilds;
use poise::serenity_prelude::GuildId;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::ErrorKind;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

pub struct App {
    pub db: DatabaseConnection,
    server_configs: RwLock<HashMap<GuildId, Arc<ServerConfig>>>,
    /// In-memory mirror of the `allowed_guilds` table, so hot paths (prefix lookup, command
    /// checks) don't have to hit the database.
    allowed_guilds: RwLock<HashSet<GuildId>>,
}

impl App {
    pub async fn new(db: DatabaseConnection) -> Result<Self, DbErr> {
        let allowed_guilds = AllowedGuilds::find()
            .all(&db)
            .await?
            .into_iter()
            .map(|guild| GuildId::new(guild.id as u64))
            .collect();

        Ok(App {
            db,
            server_configs: RwLock::new(HashMap::new()),
            allowed_guilds: RwLock::new(allowed_guilds),
        })
    }

    pub fn get_config(&self, id: GuildId) -> Arc<ServerConfig> {
        if let Some(config) = self.server_configs.read().unwrap().get(&id) {
            return Arc::clone(config);
        }

        // Newly allowed servers won't have a config file yet, fall back to the defaults
        let source = match fs::read_to_string(format!("./configs/{id}.kdl")) {
            Ok(source) => source,
            Err(e) if e.kind() == ErrorKind::NotFound => String::new(),
            Err(e) => panic!("Failed to read config for guild {id}: {e}"),
        };

        let config = Arc::new(
            knus::parse("<config>", &source)
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

    pub fn is_allowed(&self, id: GuildId) -> bool {
        self.allowed_guilds.read().unwrap().contains(&id)
    }

    pub fn allowed_guild_ids(&self) -> Vec<GuildId> {
        self.allowed_guilds.read().unwrap().iter().copied().collect()
    }

    /// Adds a guild to the allow list. Returns `false` if it was already allowed.
    pub async fn allow_guild(
        &self,
        id: GuildId,
        name: Option<String>,
        icon: Option<String>,
        owner_id: Option<i64>,
    ) -> Result<bool, DbErr> {
        if self.is_allowed(id) {
            return Ok(false);
        }

        allowed_guilds::ActiveModel {
            id: Set(id.get() as i64),
            name: Set(name.unwrap_or_else(|| "Server".into())),
            icon: Set(icon),
            owner_id: Set(owner_id.unwrap_or(0)),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        self.allowed_guilds.write().unwrap().insert(id);
        Ok(true)
    }

    /// Removes a guild from the allow list. Returns `false` if it wasn't allowed.
    pub async fn disallow_guild(&self, id: GuildId) -> Result<bool, DbErr> {
        let result = AllowedGuilds::delete_by_id(id.get() as i64)
            .exec(&self.db)
            .await?;

        self.allowed_guilds.write().unwrap().remove(&id);
        self.server_configs.write().unwrap().remove(&id);
        Ok(result.rows_affected > 0)
    }

    /// Refreshes the stored name/icon/owner of an allowed guild, so the dashboard can display it.
    pub async fn update_guild_info(
        &self,
        id: GuildId,
        name: String,
        icon: Option<String>,
        owner_id: i64,
    ) -> Result<(), DbErr> {
        if !self.is_allowed(id) {
            return Ok(());
        }

        allowed_guilds::ActiveModel {
            id: Set(id.get() as i64),
            name: Set(name),
            icon: Set(icon),
            owner_id: Set(owner_id),
            updated_at: Set(DateTimeUtc::from(SystemTime::now()).fixed_offset()),
            ..Default::default()
        }
        .update(&self.db)
        .await?;

        Ok(())
    }
}
