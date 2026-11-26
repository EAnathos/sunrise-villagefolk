//! Villagefolk gives each newly spawned villager a stable name from the server's name list.
use mod_api::{
    self, Config, ConfigFile, ConfigKind, ConfigLoadingEvent, ConfigPath, ConfigReloadingEvent,
    ConfigUnloadingEvent, EntityJoinLevelEvent, Flow, Hooks, HostSide, RegisterConfigsEvent,
    ServerHost, Text,
};
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock, RwLock};

const MOD_ID: &str = "villagefolk";
const CONFIG_FILE: &str = "villagefolk-server.txt";
const VILLAGER: &str = "minecraft:villager";
mod default_names;
use default_names::DEFAULT_NAMES;

static NAMES: OnceLock<RwLock<Vec<String>>> = OnceLock::new();
static NAMED_UUIDS: OnceLock<Mutex<HashSet<u128>>> = OnceLock::new();

fn names() -> &'static RwLock<Vec<String>> {
    NAMES.get_or_init(|| RwLock::new(parse_names(DEFAULT_NAMES)))
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
        let Ok(names) = names().read() else {
            return Flow::Continue;
        };
        if names.is_empty() {
            return Flow::Continue;
        }
        let name = &names[name_index(uuid, names.len())];
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
    let parsed = parse_names(&config.text);
    if !parsed.is_empty() {
        if let Ok(mut current) = names().write() {
            *current = parsed;
        }
    }
}

/// Each non-empty line is one name; blank lines and lines starting with `#` are ignored.
fn parse_names(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

/// Mix both halves of a UUID before selecting a name, so related UUID values are well spread.
fn name_index(uuid: u128, name_count: usize) -> usize {
    let mut value = (uuid as u64) ^ ((uuid >> 64) as u64).rotate_left(23) ^ 0x9E37_79B9_7F4A_7C15;
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    value as usize % name_count
}
