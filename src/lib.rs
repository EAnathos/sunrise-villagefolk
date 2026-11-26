//! Villagefolk gives villagers a stable, randomly selected name when they first join a world.
use mod_api::{self, EntityJoinLevelEvent, Flow, Hooks, ServerHost, Text};

const VILLAGER: &str = "minecraft:villager";
const NAMES: &[&str] = &[
    "Avery", "Alex", "Ash", "Bailey", "Basil", "Bea", "Bell", "Bennett", "Blair", "Bram", "Briar",
    "Callum", "Cedar", "Charlie", "Clement", "Clover", "Cora", "Dahlia", "Dale", "Daphne",
    "Dorian", "Eden", "Edith", "Elias", "Ellis", "Emery", "Estelle", "Fern", "Finley", "Flora",
    "Flynn", "Gale", "Gideon", "Greta", "Harper", "Hazel", "Hollis", "Iris", "Isaac", "Jasper",
    "June", "Juniper", "Kit", "Lane", "Laurel", "Linden", "Lou", "Lucille", "Mabel", "Marlow",
    "Mina", "Morgan", "Nell", "Nico", "Noel", "Oakley", "Olive", "Orla", "Otis", "Parker",
    "Perrin", "Poppy", "Quinn", "Remy", "Robin", "Rowan", "Sage", "Sasha", "Silas", "Sylvie",
    "Theodore", "Tobin", "Valerie", "Willa", "Wren", "Yara",
];

#[derive(Default)]
struct Villagefolk;

#[mod_api::sunrise_mod]
impl Hooks for Villagefolk {
    /// Give a newly spawned villager a name once. The game's entity UUID makes the selection
    /// stable for this villager, and the custom name is saved with the entity.
    fn entity_join_level(
        &self,
        host: &mut dyn ServerHost,
        event: &mut EntityJoinLevelEvent<'_>,
    ) -> Flow {
        if event.kind != VILLAGER || event.loaded_from_disk {
            return Flow::Continue;
        }

        let Some(entity) = host.entity(event.entity) else {
            return Flow::Continue;
        };
        let Some(uuid) = entity.uuid(host) else {
            return Flow::Continue;
        };
        let name = NAMES[name_index(uuid)];
        let _ = entity.set_custom_name(host, Some(Text::literal(name)));
        let _ = entity.set_custom_name_visible(host, true);
        Flow::Continue
    }
}

/// Mix both halves of a UUID before selecting a name, so related UUID values are well spread.
fn name_index(uuid: u128) -> usize {
    let mut value = (uuid as u64) ^ ((uuid >> 64) as u64).rotate_left(23) ^ 0x9E37_79B9_7F4A_7C15;
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    value as usize % NAMES.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_villager_keeps_the_same_name_selection() {
        let uuid = 0xA13F_29B7_640D_8C21_5E0A_73D4_B6F2_1098;
        assert_eq!(name_index(uuid), name_index(uuid));
        assert!(name_index(uuid) < NAMES.len());
    }

    #[test]
    fn different_uuids_are_not_all_assigned_the_same_name() {
        let indices: std::collections::HashSet<_> = (0..32).map(name_index).collect();
        assert!(indices.len() > 1);
    }
}
