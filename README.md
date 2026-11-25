# Villagefolk

Give the villagers of Sunrise a name to remember.

**Villagefolk** is a native Rust mod for [Sunrise](https://playsunrise.net) that gives villagers in villages a randomly
chosen name. The goal is to make village life feel more personal without adding new entities or changing how villagers
behave.

> Early project setup. The naming behaviour and Sunrise compatibility are not implemented yet.

## Planned

- Give naturally spawned villagers in villages a randomly selected name.
- Keep each villager's name stable across saves and reloads.
- Avoid assigning a new name each time a villager is seen or loaded.
- Keep the feature lightweight and compatible with ordinary villager behaviour.

See [SPECS.md](SPECS.md) for the initial design and open implementation questions.

## Install

There is no playable release yet. Once available, the mod will be listed on [Sunrise Mods](https://playsunrise.net/mods).

## Development

Villagefolk is intended to be a standalone native Sunrise mod, written in Rust and inspired by the structure of
[EAnathos/sunrise-creatures](https://github.com/EAnathos/sunrise-creatures). Build instructions will be added once the
mod's Sunrise API hooks and package manifest are in place.

## License

Code and text: [PolyForm Noncommercial License 1.0.0](LICENSE).
