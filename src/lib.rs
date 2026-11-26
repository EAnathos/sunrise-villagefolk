//! Villagefolk gives each newly spawned villager a stable name from the server's name list.
use mod_api::{
    self, Config, ConfigFile, ConfigKind, ConfigLoadingEvent, ConfigPath, ConfigReloadingEvent,
    ConfigUnloadingEvent, EntityJoinLevelEvent, Flow, Hooks, HostSide, RegisterConfigsEvent,
    ServerHost, Text,
};
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock, RwLock};

const MOD_ID: &str = "villagefolk";
const CONFIG_FILE: &str = "villagefolk-server.json";
const VILLAGER: &str = "minecraft:villager";
mod default_names;
use default_names::DEFAULT_NAMES;

static NAMES: OnceLock<RwLock<NameLists>> = OnceLock::new();
static NAMED_UUIDS: OnceLock<Mutex<HashSet<u128>>> = OnceLock::new();

fn names() -> &'static RwLock<NameLists> {
    NAMES.get_or_init(|| RwLock::new(parse_names(DEFAULT_NAMES).unwrap_or_default()))
}

#[derive(Default)]
struct Villagefolk;

#[mod_api::sunrise_mod]
impl Hooks for Villagefolk {
    fn register_configs(&self, _host: HostSide<'_>, event: &mut RegisterConfigsEvent<'_>) {
        event
            .configs
            .push(Config::new(MOD_ID, ConfigKind::Server, DEFAULT_NAMES).file(CONFIG_FILE));
    }

    fn config_loading(&self, _host: HostSide<'_>, event: &mut ConfigLoadingEvent<'_>) {
        load_config(&event.config);
    }

    fn config_reloading(&self, _host: HostSide<'_>, event: &mut ConfigReloadingEvent<'_>) {
        load_config(&event.config);
    }

    fn config_unloading(&self, _host: &mut dyn ServerHost, _event: &mut ConfigUnloadingEvent<'_>) {
        if let Some(cache) = NAMED_UUIDS.get() {
            if let Ok(mut cache) = cache.lock() {
                cache.clear();
            }
        }
    }

    /// Name a villager once, including when it first loads from a village chunk. World data
    /// records assignments across restarts, preserving later name-tag renames.
    fn entity_join_level(
        &self,
        host: &mut dyn ServerHost,
        event: &mut EntityJoinLevelEvent<'_>,
    ) -> Flow {
        if event.kind != VILLAGER {
            return Flow::Continue;
        }

        let Some(entity) = host.entity(event.entity) else {
            return Flow::Continue;
        };
        let Some(uuid) = entity.uuid(host) else {
            return Flow::Continue;
        };
        let named_uuids = NAMED_UUIDS.get_or_init(|| Mutex::new(HashSet::new()));
        let Ok(mut named_uuids) = named_uuids.lock() else {
            return Flow::Continue;
        };
        let record_path = ConfigPath::new(&format!("named/{uuid:032x}.txt"));
        let Ok(record_path) = record_path else {
            return Flow::Continue;
        };
        if named_uuids.contains(&uuid) || host.world_data_files().read(&record_path).is_ok() {
            named_uuids.insert(uuid);
            return Flow::Continue;
        }
        let style = host
            .merchant(entity)
            .and_then(|merchant| merchant.villager_type(host))
            .as_deref()
            .and_then(village_style);
        let Ok(names) = names().read() else {
            return Flow::Continue;
        };
        let Some(pool_size) = names.pool_size(style) else {
            return Flow::Continue;
        };
        let name = names
            .name_at(style, name_index(uuid, pool_size))
            .expect("pool_size counts the available names");
        let _ = entity.set_custom_name(host, Some(Text::literal(name.clone())));
        let _ = entity.set_custom_name_visible(host, true);
        named_uuids.insert(uuid);
        let _ = host
            .world_data_files()
            .write(&record_path, b"Villagefolk assigned this name.\n");
        Flow::Continue
    }
}

fn load_config(config: &ConfigFile) {
    if config.mod_id != MOD_ID || config.kind != ConfigKind::Server || config.file != CONFIG_FILE {
        return;
    }
    if let Some(parsed) = parse_names(&config.text) {
        if let Ok(mut current) = names().write() {
            *current = parsed;
        }
    }
}

#[derive(Default)]
struct NameLists {
    all: Vec<String>,
    plains: Vec<String>,
    desert: Vec<String>,
    savanna: Vec<String>,
    taiga: Vec<String>,
    snowy: Vec<String>,
}

impl NameLists {
    fn for_style(&self, style: Option<&str>) -> &[String] {
        match style {
            Some("plains") => &self.plains,
            Some("desert") => &self.desert,
            Some("savanna") => &self.savanna,
            Some("taiga") => &self.taiga,
            Some("snowy") => &self.snowy,
            _ => &[],
        }
    }

    fn pool_size(&self, style: Option<&str>) -> Option<usize> {
        let count = self.all.len() + self.for_style(style).len();
        (count > 0).then_some(count)
    }

    fn name_at(&self, style: Option<&str>, index: usize) -> Option<&String> {
        if index < self.all.len() {
            self.all.get(index)
        } else {
            self.for_style(style).get(index - self.all.len())
        }
    }
}

/// Parse `all` and the five vanilla village styles from the server's JSON config.
fn parse_names(text: &str) -> Option<NameLists> {
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let list = |key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    let names = NameLists {
        all: list("all"),
        plains: list("plains"),
        desert: list("desert"),
        savanna: list("savanna"),
        taiga: list("taiga"),
        snowy: list("snowy"),
    };
    names
        .pool_size(Some("plains"))
        .or_else(|| names.pool_size(Some("desert")))
        .or_else(|| names.pool_size(Some("savanna")))
        .or_else(|| names.pool_size(Some("taiga")))
        .or_else(|| names.pool_size(Some("snowy")))
        .map(|_| names)
}

/// Convert Sunrise's villager variant registry id into one of the five village styles.
fn village_style(villager_type: &str) -> Option<&'static str> {
    match villager_type.rsplit(':').next().unwrap_or(villager_type) {
        "plains" => Some("plains"),
        "desert" => Some("desert"),
        "savanna" => Some("savanna"),
        "taiga" => Some("taiga"),
        "snow" | "snowy" => Some("snowy"),
        _ => None,
    }
}

/// Mix both halves of a UUID before selecting a name, so related UUID values are well spread.
fn name_index(uuid: u128, name_count: usize) -> usize {
    let mut value = (uuid as u64) ^ ((uuid >> 64) as u64).rotate_left(23) ^ 0x9E37_79B9_7F4A_7C15;
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    value as usize % name_count
}
