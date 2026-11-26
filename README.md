# Villagefolk

Give the villagers of Sunrise a name to remember.

**Villagefolk** is a small native Rust mod for [Sunrise](https://playsunrise.net). It gives each newly spawned villager
a randomly selected name and shows that name above them. The name is derived from the villager's UUID, so it stays the
same when the world is saved and loaded again. A name tag used by the player can still rename the villager.

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
