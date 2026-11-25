# Villagefolk: specification

A small native mod for Sunrise that gives villagers in villages randomly selected names, so the people in a village
feel like individuals.

> ✅ = decided · ❓ = open question

## 0. Status

Project initialized. No game code or playable build yet.

## 1. Intended behaviour ✅

- Name villagers that belong to villages; do not add new villagers or alter their professions, trades, schedules or AI.
- Choose a name randomly from a curated name list.
- Assign a name once and preserve it when the villager is saved, unloaded and loaded again.
- Do not replace a name the player has already set with a name tag.
- Keep the feature lightweight, with no dependency on Sunrise Creatures.

## 2. Open implementation questions ❓

- Which Sunrise API event reliably identifies villagers in villages and supports setting a persistent custom name?
- Does the game already persist custom names and distinguish player-set names from mod-set names?
- Should names be drawn from one language-neutral list, or should the list follow the game's language?
- What should happen to villagers spawned outside villages, such as cured zombie villagers or villagers from spawn eggs?

## 3. Initial scope

The first playable version should implement the behaviour in §1, document the exact villager eligibility rules, and be
checked in a Sunrise instance before being listed in the registry.
