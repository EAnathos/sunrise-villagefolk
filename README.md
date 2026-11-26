# Villagefolk

Give the villagers of Sunrise a name to remember.

**Villagefolk** is a small native Rust mod for [Sunrise](https://playsunrise.net). It gives each villager it encounters
a name selected from a list and shows that name above them. The selection is derived from the villager's UUID, so it
stays the same when the world is saved and loaded again. A name tag used by the player can still rename the villager.

## Configure names

Villagefolk registers a server configuration named `villagefolk-server.txt`. Sunrise creates it from
[`src/default_names.rs`](src/default_names.rs) the first time the mod starts. On a dedicated server,
edit `config/villagefolk-server.txt`; for a particular world, edit or create
`saves/<world>/serverconfig/villagefolk-server.txt`. The world-specific file takes precedence. In single-player,
the integrated server uses these same server settings.

Put one name on each line. Blank lines and lines beginning with `#` are ignored. Changes are picked up while the
server is running. An empty list falls back to the previously loaded list. Existing villagers are named the first time
Villagefolk sees them, including villagers loaded from village chunks. The world stores which UUIDs Villagefolk has
handled, so chunk reloads and server restarts do not reset a later name-tag rename.

Villagefolk is server-side only. In multiplayer, install it on the server; players see the names through normal entity
updates and do not need the mod installed on their clients. In single-player, the local integrated server runs it.

## Install

There is no public release yet. Once available, Villagefolk will be listed on [Sunrise Mods](https://playsunrise.net/mods).

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
[EAnathos/sunrise-creatures](https://github.com/EAnathos/sunrise-creatures). See [SPECS.md](SPECS.md) for the current
scope and limitations.

## License

Code and text: [PolyForm Noncommercial License 1.0.0](LICENSE).
