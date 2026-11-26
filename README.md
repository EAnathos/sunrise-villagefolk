# Villagefolk

Give the villagers of Sunrise a name to remember.

**Villagefolk** is a small native Rust mod for [Sunrise](https://playsunrise.net). It gives each villager it encounters
a name selected from a list and shows that name above them. The selection is derived from the villager's UUID, so it
stays the same when the world is saved and loaded again. A name tag used by the player can still rename the villager.

## Configure names

Villagefolk registers a server configuration named `villagefolk-server.json`. Sunrise creates it from the bundled
defaults the first time the mod starts. On a dedicated server, edit `config/villagefolk-server.json`; for a particular
world, edit or create `saves/<world>/serverconfig/villagefolk-server.json`. The world-specific file takes precedence.
In single-player, the integrated server uses these same server settings. The mod is server-side only, so configure it
on the server in multiplayer; clients do not need to install it.

The JSON object has an `all` list plus lists for the five village styles: `plains`, `desert`, `savanna`, `taiga`, and
`snowy`. Put a name in `all` if it should be available to villagers of every style. Add names to a style's list to make
them available only to that style; its pool combines that list with `all`. For example, this makes Alex and Robin
available everywhere, while Mabel and Otis are added only to plains villagers:

```json
{
  "all": ["Alex", "Robin"],
  "plains": ["Mabel", "Otis"],
  "desert": ["Sahara", "Omar"],
  "savanna": ["Zuri"],
  "taiga": ["Cedar"],
  "snowy": ["Wren"]
}
```

You can add, remove, or replace entries in any list. Keep the file valid JSON: use double quotes around names, commas
between entries, and no comma after the last entry. The village style comes from the villager's type. If a style list is
empty, villagers of that style use names from `all` only. The bundled defaults include shared names, plus names themed
for each style; see [`src/default_names.rs`](src/default_names.rs) for the full list.

Configuration edits are picked up while the server is running. Villagefolk assigns a name the first time it sees a
villager, including villagers loaded from village chunks. That assignment stays saved across chunk reloads and server
restarts, and later name-tag renames are preserved. Changing the lists affects villagers that have not been named yet;
it does not rename villagers that already have an assignment. Updating the mod also does not overwrite an existing
server config, so merge new bundled names into your file if you want to add them to a world that already has one.

## Install

Find and install **Villagefolk** from the [Sunrise Mods registry](https://playsunrise.net/mods/villagefolk).

## Scope and in-game check

Villagefolk names ordinary villagers (`minecraft:villager`), including summoned villagers and villagers loaded from
village chunks. It does not name wandering traders or zombie villagers, and adds no entities, items, or AI. Names are
chosen independently, so two villagers can have the same name.

To check the main behavior in game:

1. Start a world with Villagefolk enabled and find a village.
2. Confirm its villagers display names above them.
3. Leave and reopen the world; existing villagers should keep their names.
4. Rename a villager with a name tag, leave and reopen the world, and confirm the rename remains.
5. Spawn a new villager and confirm it receives a name.

## Build and run

Install Sunrise's `cargo sunrise-mod` tool and Rust. The project is pinned to Sunrise build
`94256510f74e6843754e0a93fd9e4da7e45b1056`; the tool fetches its SDK into the ignored `.sunrise/` folder.

```sh
cargo sunrise-mod check
cargo sunrise-mod build
cargo sunrise-mod run --resources <prepared-resources> -- --world Villagefolk --creative
cargo test
```

The mod does not depend on Sunrise Creatures or other mods.

## Development

The project follows the native mod layout and release flow used by
[EAnathos/sunrise-creatures](https://github.com/EAnathos/sunrise-creatures).

## License

Code and text: [PolyForm Noncommercial License 1.0.0](LICENSE).
