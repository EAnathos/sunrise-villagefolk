# Villagefolk: specification

A small native mod for Sunrise that gives villagers individual names and makes village life feel more personal.

## 0. Status

Implemented; compile and in-game verification pending.

- Newly spawned `minecraft:villager` entities receive one name from the built-in list.
- The selection is based on the villager's UUID, so it is stable for that villager and needs no per-world random state.
- The custom name is saved with the villager and displayed above them.
- Villagers loaded from disk are left alone. The mod does not rename villagers already in an existing world.
- Name tags remain usable after a villager has spawned.

## 1. Scope

- No new entities, items, AI or dependencies.
- Only ordinary villagers are named; wandering traders and zombie villagers are not.
- The name pool is language-neutral and currently ships in the Rust source.
- Two villagers may receive the same name; names are selected independently.

## 2. Testing in game

1. Start a new world with Villagefolk enabled and locate a village.
2. Confirm villagers have visible names above them.
3. Leave and reopen the world; existing villagers should keep their names.
4. Use a name tag on a villager; its chosen name should change to the player's name.
5. Spawn a new villager and confirm it receives a name.
